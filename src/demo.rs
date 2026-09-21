use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DemoReply {
    #[serde(default)]
    pub result_code: i32,
    #[serde(default)]
    pub login: u64,
    #[serde(default)]
    pub password: String,
    #[serde(default)]
    pub investor: String,
    #[serde(default)]
    pub server: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectExReply {
    pub terminal_instance_guid: String,
    pub terminal_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisconnectReply {
    pub unique_identifier: String,
    pub full_life_time_seconds: i32,
}

pub struct DemoAccountClient {
    pub endpoint: String,
    client: reqwest::Client,
}

impl DemoAccountClient {
    pub fn new(endpoint: &str) -> Self {
        let clean = endpoint
            .trim_start_matches("http://")
            .trim_start_matches("https://")
            .trim_end_matches(":443")
            .trim_end_matches('/');
        Self {
            endpoint: format!("https://{}", clean),
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(180))
                .tcp_keepalive(std::time::Duration::from_secs(30))
                .build()
                .unwrap_or_default(),
        }
    }

    pub async fn open_demo_account(&self, server: &str, api_key: &str) -> Result<DemoReply, Box<dyn std::error::Error>> {
        for attempt in 1..=5 {
            let url = format!("{}/DemoAccount/Open", self.endpoint);
            let resp = self.client.get(&url)
                .query(&[("server", server)])
                .header("APIKey", api_key)
                .header("User-Agent", "RustCopier/1.0.0")
                .send()
                .await;

            if let Ok(r) = resp {
                if let Ok(text) = r.text().await {
                    if let Ok(v) = serde_json::from_str::<serde_json::Value>(&text) {
                        let login_str = v.get("login").and_then(|l| {
                            if l.is_string() {
                                l.as_str().map(|s| s.to_string())
                            } else if l.is_number() {
                                l.as_u64().map(|n| n.to_string())
                            } else {
                                None
                            }
                        }).unwrap_or_default();

                        let password = v.get("password").and_then(|p| p.as_str()).unwrap_or("").to_string();
                        let investor = v.get("investor").and_then(|p| p.as_str()).unwrap_or("").to_string();
                        let srv = v.get("server").and_then(|p| p.as_str()).unwrap_or(server).to_string();
                        let result_code = v.get("resultCode").and_then(|c| c.as_i64()).unwrap_or(0) as i32;

                        if let Ok(login) = login_str.parse::<u64>() {
                            if login > 0 && !password.is_empty() {
                                return Ok(DemoReply {
                                    result_code,
                                    login,
                                    password,
                                    investor,
                                    server: srv,
                                });
                            }
                        }
                    }
                }
            }
            tokio::time::sleep(std::time::Duration::from_millis(1500)).await;
        }
        Err("Failed to provision demo account after 5 attempts".into())
    }

    pub async fn connect_ex(&self, user: u64, password: &str, server: &str, api_key: &str) -> Result<ConnectExReply, Box<dyn std::error::Error>> {
        let url = format!("{}/ConnectEx", self.endpoint);
        let resp = self.client.get(&url)
            .query(&[
                ("user", user.to_string().as_str()),
                ("password", password),
                ("mtClusterName", server),
            ])
            .header("APIKey", api_key)
            .header("User-Agent", "RustCopier/1.0.0")
            .send()
            .await?;

        let text = resp.text().await?;
        if let Ok(v) = serde_json::from_str::<serde_json::Value>(&text) {
            let guid = v.get("data")
                .and_then(|d| d.get("terminalInstanceGuid"))
                .and_then(|g| g.as_str())
                .unwrap_or("")
                .to_string();
            let term_type = v.get("data")
                .and_then(|d| d.get("terminalType"))
                .and_then(|t| t.as_str())
                .unwrap_or("MT5")
                .to_string();
            if !guid.is_empty() {
                return Ok(ConnectExReply {
                    terminal_instance_guid: guid,
                    terminal_type: term_type,
                });
            }
            if let Some(err_msg) = v.get("message").and_then(|m| m.as_str()) {
                return Err(format!("ConnectEx error: {}", err_msg).into());
            }
        }
        Err(format!("Invalid ConnectEx response: {}", text).into())
    }

    pub async fn disconnect(&self, terminal_id: &str, api_key: &str) -> Result<DisconnectReply, Box<dyn std::error::Error>> {
        let url = format!("{}/Disconnect", self.endpoint);
        let resp = self.client.get(&url)
            .header("APIKey", api_key)
            .header("id", terminal_id)
            .header("User-Agent", "RustCopier/1.0.0")
            .send()
            .await?;

        let text = resp.text().await?;
        if let Ok(v) = serde_json::from_str::<serde_json::Value>(&text) {
            let uid = v.get("data")
                .and_then(|d| d.get("uniqueIdentifier"))
                .and_then(|u| u.as_str())
                .unwrap_or(terminal_id)
                .to_string();
            let life = v.get("data")
                .and_then(|d| d.get("fullLifeTimeSeconds"))
                .and_then(|l| l.as_i64())
                .unwrap_or(0) as i32;
            return Ok(DisconnectReply {
                unique_identifier: uid,
                full_life_time_seconds: life,
            });
        }
        Ok(DisconnectReply {
            unique_identifier: terminal_id.to_string(),
            full_life_time_seconds: 0,
        })
    }

    pub async fn order_send(&self, terminal_id: &str, symbol: &str, operation: &str, volume: f64, api_key: &str) -> Result<u64, Box<dyn std::error::Error>> {
        let url = format!("{}/OrderSend", self.endpoint);
        let resp = self.client.get(&url)
            .query(&[
                ("id", terminal_id),
                ("symbol", symbol),
                ("operation", operation),
                ("volume", &format!("{:.2}", volume)),
                ("stoploss", "0"),
                ("takeprofit", "0"),
                ("comment", "RustCopier_Test"),
            ])
            .header("APIKey", api_key)
            .header("id", terminal_id)
            .header("User-Agent", "RustCopier/1.0.0")
            .send()
            .await?;
        let text = resp.text().await?;
        if let Ok(v) = serde_json::from_str::<serde_json::Value>(&text) {
            if let Some(d) = v.get("data") {
                if let Some(t) = d.get("order").or_else(|| d.get("ticket")).and_then(|t| t.as_u64()) {
                    return Ok(t);
                }
            }
            if let Some(t) = v.get("order").or_else(|| v.get("ticket")).and_then(|t| t.as_u64()) {
                return Ok(t);
            }
            if let Some(n) = v.as_u64() {
                return Ok(n);
            }
        }
        if let Ok(n) = text.trim().parse::<u64>() {
            return Ok(n);
        }
        Ok(0)
    }

    pub async fn opened_orders(&self, terminal_id: &str, api_key: &str) -> Result<Vec<Position>, Box<dyn std::error::Error>> {
        let url = format!("{}/OpenedOrders?id={}", self.endpoint, terminal_id);
        let resp = self.client.get(&url)
            .header("APIKey", api_key)
            .header("id", terminal_id)
            .header("User-Agent", "RustCopier/1.0.0")
            .send()
            .await?;
        let text = resp.text().await?;
        if let Ok(positions) = serde_json::from_str::<Vec<Position>>(&text) {
            return Ok(positions);
        }
        #[derive(Deserialize)]
        struct Wrapper {
            #[serde(default)]
            data: Inner,
        }
        #[derive(Deserialize, Default)]
        struct Inner {
            #[serde(default, rename = "positionInfos")]
            position_infos: Vec<Position>,
        }
        if let Ok(w) = serde_json::from_str::<Wrapper>(&text) {
            if !w.data.position_infos.is_empty() {
                return Ok(w.data.position_infos);
            }
        }
        #[derive(Deserialize)]
        struct WrapperList {
            #[serde(default)]
            data: Vec<Position>,
        }
        if let Ok(w) = serde_json::from_str::<WrapperList>(&text) {
            return Ok(w.data);
        }
        Ok(vec![])
    }

    pub async fn order_close(&self, terminal_id: &str, ticket: u64, api_key: &str) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!("{}/OrderClose", self.endpoint);
        let resp = self.client.get(&url)
            .query(&[
                ("id", terminal_id),
                ("ticket", ticket.to_string().as_str()),
                ("volume", "0"),
                ("slippage", "20"),
            ])
            .header("APIKey", api_key)
            .header("id", terminal_id)
            .header("User-Agent", "RustCopier/1.0.0")
            .send()
            .await?;
        Ok(resp.text().await?)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    #[serde(rename = "ticket", alias = "Ticket", default)]
    pub ticket: u64,
    #[serde(rename = "symbol", alias = "Symbol", default)]
    pub symbol: String,
    #[serde(rename = "volume", alias = "Volume", default)]
    pub volume: f64,
    #[serde(rename = "type", alias = "Type", default)]
    pub position_type: String,
}

pub fn to_hyphen_guid(guid: &str) -> String {
    let clean = guid.trim_start_matches("mt5_live_").replace('-', "");
    if clean.len() == 32 {
        format!("{}-{}-{}-{}-{}", &clean[0..8], &clean[8..12], &clean[12..16], &clean[16..20], &clean[20..32])
    } else {
        guid.to_string()
    }
}

