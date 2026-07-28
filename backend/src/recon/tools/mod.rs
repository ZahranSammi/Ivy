use std::process::Command;

pub async fn run_tool(tool_id: &str, arguments: serde_json::Value) -> anyhow::Result<String> {
    match tool_id {
        "subfinder" => {
            let domain = arguments["domain"]
                .as_str()
                .ok_or_else(|| anyhow::anyhow!("Argumen 'domain' wajib diisi untuk subfinder"))?;
            run_subfinder(domain).await
        }

        "rustscan" => {
            let addresses = arguments["addresses"]
                .as_str()
                .ok_or_else(|| anyhow::anyhow!("Argumen 'addresses' wajib diisi untuk rustscan"))?;
            run_rustscan(addresses).await
        }
        _ => anyhow::bail!("tool {} tidak di dukung", tool_id),
    }
}

//fungsi subfinder
async fn run_subfinder(domain: &str) -> anyhow::Result<String> {
    let output = Command::new("subfinder")
        .args(["-d", domain, "-silent"])
        .output()?;

    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Subfinder gagal: {}", error_msg);
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

// Fungsi Rustscan
async fn run_rustscan(addresses: &str) -> anyhow::Result<String> {
    // Jalankan: rustscan -a <addresses> --
    let output = Command::new("rustscan")
        .args(["-a", addresses, "--timeout", "2000"])
        .output()?;

    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Rustscan gagal: {}", error_msg);
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
