use rustcopier::client::CopierService;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let key = std::env::var("MRPC_API_KEY")?;
    let service = CopierService::connect("https://copy.mrpc.pro", &key).await?;
    let result = service.list().await?;
    if !result.ok { return Err(result.error.into()); }
    for copier in result.copiers { println!("{}: paused={}", copier.id, copier.paused); }
    Ok(())
}
