use rustcopier::{CopierService, StartRequest, Account};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== RustCopier Quick Start ===");
    let client = CopierService::connect("https://copy.mrpc.pro:443", "YOUR_USER_KEY", "").await?;
    let reply = client.start(StartRequest {
        user_key: "YOUR_USER_KEY".into(),
        manager_key: "YOUR_USER_KEY".into(),
        master: Account { r#type: "MT5".into(), user: 10001, password: "x".into(), server: "MetaQuotes-Demo".into(), name: "Master".into() },
        slave: Account { r#type: "MT5".into(), user: 10002, password: "y".into(), server: "MetaQuotes-Demo".into(), name: "Slave".into() },
        risk_type: "LotMultiplier".into(),
        risk_value: "1.5".into(),
        fixed_master_balance: "".into(),
        copy_sl: true,
        copy_tp: true,
        copy_pending_orders: false,
        reverse_copy: false,
    }).await?;
    println!("Started copier ID: {}", reply.copier_id);
    Ok(())
}
