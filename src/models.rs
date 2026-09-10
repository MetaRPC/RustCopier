use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub r#type: String,
    pub user: u64,
    pub password: String,
    pub server: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartRequest {
    pub user_key: String,
    pub manager_key: String,
    pub master: Account,
    pub slave: Account,
    pub risk_type: String,
    pub risk_value: String,
    pub fixed_master_balance: String,
    pub copy_sl: bool,
    pub copy_tp: bool,
    pub copy_pending_orders: bool,
    pub reverse_copy: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartReply {
    pub ok: bool,
    pub copier_id: String,
    pub error: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopierSummary {
    pub id: String,
    pub master_type: String,
    pub master_user: u64,
    pub master_server: String,
    pub slave_type: String,
    pub slave_user: u64,
    pub slave_server: String,
    pub risk_type: String,
    pub risk_value: String,
    pub paused: bool,
    pub pause_reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListReply {
    pub ok: bool,
    pub copiers: Vec<CopierSummary>,
    pub error: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleReply {
    pub ok: bool,
    pub error: String,
}
