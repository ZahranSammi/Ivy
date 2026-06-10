use std::net::{IpAddr, SocketAddr, ToSocketAddrs};
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::timeout;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use clap::Parser;
use chrono::Utc;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    domain: String,

    #[arg(short, long)]
    job_id: String,

    #[arg(long, default_value_t = 1800)] // 30 minutes default watchdog
    watchdog_seconds: u64,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
struct Subdomain {
    name: String,
    source: String,
    is_wildcard: bool,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
struct Port {
    number: u16,
    state: String,
    protocol: String,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
struct Host {
    ip: String,
    asn: Option<i64>,
    ports: Vec<Port>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
struct Service {
    host: String,
    port: u16,
    name: Option<String>,
    version: Option<String>,
    http_status: Option<i64>,
    server_header: Option<String>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
struct ExposedFile {
    url: String,
    path: String,
    http_status: i64,
    content_length: i64,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
struct Findings {
    subdomains: Vec<Subdomain>,
    hosts: Vec<Host>,
    services: Vec<Service>,
    exposed_files: Vec<ExposedFile>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
struct ReconError {
    stage: String,
    target: String,
    message: String,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
struct ReconResult {
    schema_version: String,
    job_id: String,
    status: String,
    scanned_at: String,
    findings: Findings,
    errors: Vec<ReconError>,
}

#[derive(Deserialize, Debug)]
struct CrtShEntry {
    name_value: String,
}

async fn query_crt_sh(domain: &str) -> Result<Vec<String>, reqwest::Error> {
    let url = format!("https://crt.sh/?q=%25.{}&output=json", domain);
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()?;
    
    let response = client.get(&url).send().await?;
    let entries: Vec<CrtShEntry> = response.json().await?;
    
    let mut subdomains = Vec::new();
    for entry in entries {
        for name in entry.name_value.split('\n') {
            let clean_name = name.trim().to_string();
            if !clean_name.is_empty() && !subdomains.contains(&clean_name) {
                subdomains.push(clean_name);
            }
        }
    }
    Ok(subdomains)
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    
    // NFR-19: Watchdog timer
    let watchdog_sec = args.watchdog_seconds;
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(watchdog_sec)).await;
        eprintln!("[WATCHDOG] Self-timeout triggered after {} seconds. Exiting.", watchdog_sec);
        std::process::exit(1);
    });

    let mut result = ReconResult {
        schema_version: "1.0".to_string(),
        job_id: args.job_id.clone(),
        status: "complete".to_string(),
        scanned_at: Utc::now().to_rfc3339(),
        findings: Findings {
            subdomains: Vec::new(),
            hosts: Vec::new(),
            services: Vec::new(),
            exposed_files: Vec::new(),
        },
        errors: Vec::new(),
    };

    // 1. Subdomain Enum
    let mut detected_subdomains = vec![args.domain.clone()];
    match query_crt_sh(&args.domain).await {
        Ok(subs) => {
            for sub in subs {
                if !detected_subdomains.contains(&sub) {
                    detected_subdomains.push(sub);
                }
            }
        }
        Err(e) => {
            result.status = "partial".to_string();
            result.errors.push(ReconError {
                stage: "subdomain_enum".to_string(),
                target: args.domain.clone(),
                message: format!("crt.sh query failed: {}", e),
            });
        }
    }

    for sub in &detected_subdomains {
        result.findings.subdomains.push(Subdomain {
            name: sub.clone(),
            source: "crt.sh".to_string(),
            is_wildcard: sub.starts_with('*'),
        });
    }

    // 2. DNS Resolution & Port Scan
    // Let's resolve the main domain and subdomains to IPs, then port scan them.
    // For local testing safety, we only scan top ports.
    let target_ports = vec![80, 443, 8080, 22, 8000, 3000]; // limited set for speed & safety

    let mut resolved_ips: Vec<IpAddr> = Vec::new();
    for sub in &detected_subdomains {
        if sub.starts_with('*') {
            continue;
        }
        let socket_str = format!("{}:80", sub);
        if let Ok(addrs) = socket_str.to_socket_addrs() {
            for addr in addrs {
                let ip = addr.ip();
                if !resolved_ips.contains(&ip) {
                    resolved_ips.push(ip);
                }
            }
        }
    }

    // Fallback if no subdomains resolved to IPs
    if resolved_ips.is_empty() {
        if let Ok(addrs) = format!("{}:80", args.domain).to_socket_addrs() {
            for addr in addrs {
                resolved_ips.push(addr.ip());
            }
        }
    }

    let mut scan_tasks = Vec::new();

    for ip in resolved_ips.clone() {
        for port in &target_ports {
            let port = *port;
            let ip_str = ip.to_string();
            scan_tasks.push(tokio::spawn(async move {
                let socket_addr = SocketAddr::new(ip, port);
                let check = timeout(Duration::from_millis(300), TcpStream::connect(&socket_addr)).await;
                
                let mut found_port = None;
                let mut found_service = None;
                let mut found_exposed_files = Vec::new();

                if let Ok(Ok(_stream)) = check {
                    found_port = Some(Port {
                        number: port,
                        state: "open".to_string(),
                        protocol: "tcp".to_string(),
                    });

                    // Add to service detection
                    let mut service = Service {
                        host: ip_str.clone(),
                        port,
                        name: Some(match port {
                            80 | 8080 | 8000 | 3000 => "http".to_string(),
                            443 => "https".to_string(),
                            22 => "ssh".to_string(),
                            _ => "unknown".to_string(),
                        }),
                        version: None,
                        http_status: None,
                        server_header: None,
                    };

                    // If HTTP/HTTPS port, let's probe it
                    if port == 80 || port == 443 || port == 8080 || port == 8000 || port == 3000 {
                        let proto = if port == 443 { "https" } else { "http" };
                        let client = reqwest::Client::builder()
                            .timeout(Duration::from_secs(3))
                            .danger_accept_invalid_certs(true)
                            .build();

                        if let Ok(c) = client {
                            let url = format!("{}://{}:{}", proto, ip_str, port);
                            if let Ok(resp) = c.get(&url).send().await {
                                service.http_status = Some(resp.status().as_u16() as i64);
                                if let Some(server) = resp.headers().get("Server") {
                                    if let Ok(val) = server.to_str() {
                                        service.server_header = Some(val.to_string());
                                    }
                                }

                                // Probe sensitive files
                                for path in &["/.env", "/.git"] {
                                    let file_url = format!("{}{}", url, path);
                                    if let Ok(file_resp) = c.get(&file_url).send().await {
                                        let status = file_resp.status().as_u16() as i64;
                                        if status == 200 || status == 403 || status == 301 || status == 302 {
                                            let content_len = file_resp.headers()
                                                .get("content-length")
                                                .and_then(|v| v.to_str().ok())
                                                .and_then(|v| v.parse::<i64>().ok())
                                                .unwrap_or(0);
                                            found_exposed_files.push(ExposedFile {
                                                url: file_url.clone(),
                                                path: path.to_string(),
                                                http_status: status,
                                                content_length: content_len,
                                            });
                                        }
                                    }
                                }
                            }
                        }
                    }
                    found_service = Some(service);
                }
                (ip_str, found_port, found_service, found_exposed_files)
            }));
        }
    }

    let mut scan_results = Vec::new();
    for task in scan_tasks {
        if let Ok(res) = task.await {
            scan_results.push(res);
        }
    }

    for ip in resolved_ips {
        let ip_str = ip.to_string();
        let mut host_ports = Vec::new();

        for (res_ip_str, found_port, found_service, exposed_files) in &scan_results {
            if res_ip_str == &ip_str {
                if let Some(port) = found_port {
                    host_ports.push(port.clone());
                }
                if let Some(service) = found_service {
                    result.findings.services.push(service.clone());
                }
                for file in exposed_files {
                    result.findings.exposed_files.push(file.clone());
                }
            }
        }

        result.findings.hosts.push(Host {
            ip: ip_str,
            asn: None,
            ports: host_ports,
        });
    }

    // Print findings to stdout
    let output_json = serde_json::to_string_pretty(&result).unwrap();
    println!("{}", output_json);
}
