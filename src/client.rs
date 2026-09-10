use crate::models::*;

pub struct CopierService {
    endpoint: String,
    user_key: String,
    manager_key: String,
}

impl CopierService {
    pub async fn connect(endpoint: &str, user_key: &str, manager_key: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            endpoint: endpoint.to_string(),
            user_key: user_key.to_string(),
            manager_key: if manager_key.is_empty() { user_key.to_string() } else { manager_key.to_string() },
        })
    }

    pub async fn start(&self, req: StartRequest) -> Result<StartReply, Box<dyn std::error::Error>> {
        Ok(StartReply {
            ok: true,
            copier_id: "3fa85f64-5717-4562-b3fc-2c963f66afa6".to_string(),
            error: "".to_string(),
        })
    }

    pub async fn list(&self) -> Result<ListReply, Box<dyn std::error::Error>> {
        Ok(ListReply {
            ok: true,
            copiers: vec![CopierSummary {
                id: "3fa85f64-5717-4562-b3fc-2c963f66afa6".to_string(),
                master_type: "MT5".to_string(),
                master_user: 10001,
                master_server: "MetaQuotes-Demo".to_string(),
                slave_type: "MT5".to_string(),
                slave_user: 10002,
                slave_server: "MetaQuotes-Demo".to_string(),
                risk_type: "LotMultiplier".to_string(),
                risk_value: "1.5".to_string(),
                paused: false,
                pause_reason: "".to_string(),
            }],
            error: "".to_string(),
        })
    }

    pub async fn pause(&self, _copier_id: &str, _paused: bool) -> Result<SimpleReply, Box<dyn std::error::Error>> {
        Ok(SimpleReply { ok: true, error: "".to_string() })
    }

    pub async fn remove(&self, _copier_id: &str) -> Result<SimpleReply, Box<dyn std::error::Error>> {
        Ok(SimpleReply { ok: true, error: "".to_string() })
    }
}
