pub mod tools;

pub struct ExecutionPlan {
    pub tool_ids: Vec<String>,
}

// Fungsi untuk perencanaan & eksekusi passive recon (Subfinder)
pub async fn plan_passive_recon(target: &str) -> anyhow::Result<ExecutionPlan> {
    let mut executed_tools = Vec::new();

    // Menyiapkan argumen JSON untuk subfinder
    let subfinder_args = serde_json::json!({
        "domain": target
    });

    println!(
        "[*] Memulai passive scan
  (Subfinder) pada target: {}",
        target
    );

    // Memanggil run_tool dari sub-modul tools
    use crate::recon::tools;
    match tools::run_tool("subfinder", subfinder_args).await {
        Ok(output) => {
            println!(
                "[+] Subfinder Berhasil!    
  Hasil output:\n{}",
                output
            );
            executed_tools.push("subfinder".to_string());
        }
        Err(e) => {
            println!("[-] Subfinder Gagal: {}", e);
            anyhow::bail!(
                "Gagal mengeksekusi    
  Subfinder: {}",
                e
            );
        }
    }

    Ok(ExecutionPlan {
        tool_ids: executed_tools,
    })
}

// Fungsi untuk perencanaan & eksekusi active recon (Rustscan)
pub async fn plan_active_recon(target: &str) -> anyhow::Result<ExecutionPlan> {
    let mut executed_tools = Vec::new();

    // Menyiapkan argumen JSON untuk rustscan
    let rustscan_args = serde_json::json!({
        "addresses": target
    });

    println!(
        "[*] Memulai active scan
  (Rustscan) pada target: {}",
        target
    );

    // Memanggil run_tool dari sub-modul tools
    use crate::recon::tools;
    match tools::run_tool("rustscan", rustscan_args).await {
        Ok(output) => {
            println!(
                "[+] Rustscan Berhasil!     
  Hasil output:\n{}",
                output
            );
            executed_tools.push("rustscan".to_string());
        }
        Err(e) => {
            println!("[-] Rustscan Gagal: {}", e);
            anyhow::bail!(
                "Gagal mengeksekusi    
  Rustscan: {}",
                e
            );
        }
    }

    Ok(ExecutionPlan {
        tool_ids: executed_tools,
    })
}
