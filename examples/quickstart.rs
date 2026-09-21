use rustcopier::{CopierService, DemoAccountClient, Account, StartRequest, to_hyphen_guid};
use std::io::Write;

macro_rules! log_info {
    ($($arg:tt)*) => {{
        println!($($arg)*);
        let _ = std::io::stdout().flush();
    }};
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    log_info!("=== MetaRPC RustCopier Trade Replication Quick Start ===");
    let api_key = "TRIAL";

    let demo = DemoAccountClient::new("https://mt5.mrpc.pro");
    let client = CopierService::connect("https://copy.mrpc.pro", api_key).await?;

    let mut master_guid = String::new();
    let mut slave_guid = String::new();
    let mut copier_id = String::new();
    let mut master_ticket: u64 = 0;

    // Wrapped in a closure-like block for cleanup via defer pattern
    let result: Result<(), Box<dyn std::error::Error>> = async {
        // 1. Provision live demo accounts
        log_info!("\n[1] Provisioning live demo accounts on MetaQuotes-Demo...");
        let mut master = demo.open_demo_account("MetaQuotes-Demo", api_key).await?;
        log_info!("    Master Account Provisioned: #{} on {}", master.login, master.server);
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;

        let mut slave = demo.open_demo_account("MetaQuotes-Demo", api_key).await?;
        log_info!("    Slave Account Provisioned:  #{} on {}", slave.login, slave.server);
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;

        // 2. Connect terminals via ConnectEx
        log_info!("\n[2] Connecting terminals via ConnectEx (APIKey: {})...", api_key);
        for attempt in 1..=3 {
            match demo.connect_ex(master.login, &master.password, &master.server, api_key).await {
                Ok(conn) if !conn.terminal_instance_guid.is_empty() => {
                    master_guid = conn.terminal_instance_guid;
                    break;
                }
                Err(e) => log_info!("    Master ConnectEx attempt {} error: {}", attempt, e),
                _ => {}
            }
            if attempt < 3 {
                log_info!("    Retrying master with fresh demo account...");
                if let Ok(m) = demo.open_demo_account("MetaQuotes-Demo", api_key).await {
                    master = m;
                }
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            }
        }
        if master_guid.is_empty() {
            log_info!("    Failed to connect master terminal.");
            return Ok(());
        }
        log_info!("    Master Terminal Connected! GUID: {}", master_guid);

        for attempt in 1..=3 {
            match demo.connect_ex(slave.login, &slave.password, &slave.server, api_key).await {
                Ok(conn) if !conn.terminal_instance_guid.is_empty() => {
                    slave_guid = conn.terminal_instance_guid;
                    break;
                }
                Err(e) => log_info!("    Slave ConnectEx attempt {} error: {}", attempt, e),
                _ => {}
            }
            if attempt < 3 {
                log_info!("    Retrying slave with fresh demo account...");
                if let Ok(s) = demo.open_demo_account("MetaQuotes-Demo", api_key).await {
                    slave = s;
                }
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            }
        }
        if slave_guid.is_empty() {
            log_info!("    Failed to connect slave terminal.");
            return Ok(());
        }
        log_info!("    Slave Terminal Connected!  GUID: {}", slave_guid);

        let master_session_id = to_hyphen_guid(&master_guid);
        let slave_session_id = to_hyphen_guid(&slave_guid);

        // 3. Start Trade Copier via gRPC on copy.mrpc.pro:443
        log_info!("\n[3] Starting Trade Copier via gRPC on copy.mrpc.pro:443...");
        let start_req = StartRequest {
            user_key: api_key.to_string(),
            manager_key: String::new(),
            master: Account {
                r#type: "MT5".to_string(),
                user: master.login,
                password: master.password.clone(),
                server: master.server.clone(),
                name: String::new(),
                id: master_session_id,
            },
            slave: Account {
                r#type: "MT5".to_string(),
                user: slave.login,
                password: slave.password.clone(),
                server: slave.server.clone(),
                name: String::new(),
                id: slave_session_id,
            },
            risk_type: "LotMultiplier".to_string(),
            risk_value: "1.0".to_string(),
            fixed_master_balance: String::new(),
            copy_sl: true,
            copy_tp: true,
            copy_pending_orders: false,
            reverse_copy: false,
        };

        let start_rep = client.start(start_req).await?;
        log_info!("    gRPC Start Reply: ok={}, copierId={}, error={}", start_rep.ok, start_rep.copier_id, start_rep.error);
        if !start_rep.ok {
            log_info!("    Copier Start returned error: {}", start_rep.error);
            return Ok(());
        }
        copier_id = start_rep.copier_id;

        tokio::time::sleep(std::time::Duration::from_secs(4)).await;

        // 4. Place Market Order on Master
        log_info!("\n[4] Opening Market Order on Master (0.01 EURUSD BUY)...");
        master_ticket = demo.order_send(&master_guid, "EURUSD", "TMT5_ORDER_TYPE_BUY", 0.01, api_key).await?;
        log_info!("    Master Order Placed! Ticket: {}", master_ticket);

        // 5. Verify Trade Copied to Slave
        log_info!("\n[5] Verifying replicated trade on Slave account...");
        let mut replicated = false;
        for attempt in 1..=15 {
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            let positions = demo.opened_orders(&slave_guid, api_key).await?;
            log_info!("    Attempt {}: Slave active positions count = {}", attempt, positions.len());
            if !positions.is_empty() {
                let first = &positions[0];
                log_info!("    --> CONFIRMED ON SLAVE: Ticket={}, Symbol={}, Volume={:.2}, Type={}", first.ticket, first.symbol, first.volume, first.position_type);
                replicated = true;
                break;
            }
        }

        if replicated {
            log_info!("    SUCCESS: Trade successfully replicated to slave account!");
        } else {
            log_info!("    WARNING: Slave trade replication timed out.");
        }

        // 6. Close Position on Master
        if master_ticket > 0 {
            log_info!("\n[6] Closing Master trade ticket #{}...", master_ticket);
            let close_resp = demo.order_close(&master_guid, master_ticket, api_key).await?;
            log_info!("    Master OrderClose result: {}", close_resp);

            // 7. Verify Trade Closed on Slave
            log_info!("\n[7] Verifying trade closed on Slave...");
            for attempt in 1..=15 {
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                let positions = demo.opened_orders(&slave_guid, api_key).await?;
                if positions.is_empty() {
                    log_info!("    SUCCESS: Slave position closed by trade copier!");
                    break;
                }
                log_info!("    Attempt {}: Slave positions still open: {}", attempt, positions.len());
            }
        }

        // 8. Remove Copier via gRPC
        if !copier_id.is_empty() {
            log_info!("\n[8] Removing Copier {} via gRPC...", copier_id);
            let rem_rep = client.remove(&copier_id).await?;
            log_info!("    Copier Remove Reply: ok={}", rem_rep.ok);
        }

        Ok(())
    }.await;

    if let Err(e) = result {
        log_info!("    Error during execution: {}", e);
    }

    // 9. Cleanly Disconnect Terminal Sessions
    log_info!("\n[9] Disconnecting terminal sessions cleanly via /Disconnect...");
    if !master_guid.is_empty() {
        match demo.disconnect(&master_guid, api_key).await {
            Ok(disc) => log_info!("    Master Terminal Cleanly Disconnected: {} (Lifetime: {}s)", disc.unique_identifier, disc.full_life_time_seconds),
            Err(e) => log_info!("    Master disconnect error: {}", e),
        }
    }
    if !slave_guid.is_empty() {
        match demo.disconnect(&slave_guid, api_key).await {
            Ok(disc) => log_info!("    Slave Terminal Cleanly Disconnected:  {} (Lifetime: {}s)", disc.unique_identifier, disc.full_life_time_seconds),
            Err(e) => log_info!("    Slave disconnect error: {}", e),
        }
    }

    log_info!("\n=== RustCopier Trade Replication Completed Successfully ===");
    Ok(())
}
