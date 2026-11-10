use rustcopier::{
    client::CopierService,
    models::{Account, StartRequest},
};
use serde_json::json;
use std::io::{self, BufRead};
#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        println!("{}", json!({"ok":false,"error":e.to_string()}));
        std::process::exit(1)
    }
}
async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let v: Vec<String> = io::stdin()
        .lock()
        .lines()
        .take(13)
        .collect::<Result<_, _>>()?;
    if v.len() != 13 {
        return Err("Expected 13 configuration lines".into());
    }
    let args: Vec<String> = std::env::args().collect();
    let c = CopierService::connect(&v[0], &v[1]).await?;
    let acc = |i: usize| -> Result<Account, Box<dyn std::error::Error>> {
        Ok(Account {
            r#type: "MT5".into(),
            user: v[i].parse()?,
            password: v[i + 1].clone(),
            server: v[i + 2].clone(),
            id: v[i + 3].clone(),
            name: String::new(),
        })
    };
    let r = match args[1].as_str() {
        "start" => serde_json::to_value(
            c.start(StartRequest {
                user_key: String::new(),
                manager_key: String::new(),
                master: acc(2)?,
                slave: acc(6)?,
                risk_type: v[10].clone(),
                risk_value: v[11].clone(),
                fixed_master_balance: v[12].clone(),
                copy_sl: true,
                copy_tp: true,
                copy_pending_orders: false,
                reverse_copy: false,
            })
            .await?,
        )?,
        "list" => serde_json::to_value(c.list().await?)?,
        "pause" => serde_json::to_value(c.pause(&args[2], args[3] == "true").await?)?,
        "remove" => serde_json::to_value(c.remove(&args[2]).await?)?,
        _ => return Err("Unknown operation".into()),
    };
    println!("{}", r);
    Ok(())
}
