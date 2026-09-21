use rustcopier::{CopierService, DemoAccountClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== RustCopier Quick Start Demo ===");
    let api_key = "TRIAL";

    let demo = DemoAccountClient::new("https://mt5.mrpc.pro");

    // 1. Provision live demo account
    println!("\n[1] Provisioning live demo account on MetaQuotes-Demo...");
    let master = demo.open_demo_account("MetaQuotes-Demo", api_key)?;
    println!("    Master Account Provisioned: #{} on {}", master.login, master.server);

    // 2. Connect terminal via ConnectEx with APIKey: TRIAL
    println!("\n[2] Connecting terminal via ConnectEx (APIKey: {})...", api_key);
    let conn = demo.connect_ex(master.login, &master.password, &master.server, api_key)?;
    println!("    Terminal Connected! Instance GUID: {}", conn.terminal_instance_guid);

    // 3. Interacting with Copier Service
    println!("\n[3] Interacting with Copier Service (user_key: {})...", api_key);
    let client = CopierService::connect("https://copy.mrpc.pro:443", api_key).await?;
    let list = client.list().await?;
    println!("    Active copiers count: {}", list.copiers.len());

    // 4. Cleanly Disconnect Terminal Session
    println!("\n[4] Disconnecting terminal session {}...", conn.terminal_instance_guid);
    let disc = demo.disconnect(&conn.terminal_instance_guid, api_key)?;
    println!("    Terminal Cleanly Disconnected: {} (Lifetime: {}s)", disc.unique_identifier, disc.full_life_time_seconds);

    println!("\n=== RustCopier Quick Start Completed Successfully ===");
    Ok(())
}
