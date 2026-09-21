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
                .timeout(std::time::Duration::from_secs(120))
                .build()
                .unwrap_or_default(),
        }
    }

    pub async fn open_demo_account(&self, server: &str, api_key: &str) -> Result<DemoReply, Box<dyn std::error::Error>> {
        let url = format!("{}/DemoAccount/Open", self.endpoint);
        let resp = self.client.get(&url)
            .query(&[("server", server)])
            .header("APIKey", api_key)
            .header("User-Agent", "RustCopier/1.0.0")
            .send()
            .await?;

        #[derive(Deserialize)]
        struct RawDemo {
            #[serde(rename = "resultCode", default)]
            result_code: i32,
            login: String,
            password: String,
            #[serde(default)]
            investor: String,
            #[serde(default)]
            server: String,
        }

        let raw: RawDemo = resp.json().await?;
        Ok(DemoReply {
            result_code: raw.result_code,
            login: raw.login.parse::<u64>().unwrap_or(0),
            password: raw.password,
            investor: raw.investor,
            server: if raw.server.is_empty() { server.to_string() } else { raw.server },
        })
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

        #[derive(Deserialize)]
        struct Data {
            #[serde(rename = "terminalInstanceGuid")]
            terminal_instance_guid: String,
            #[serde(rename = "terminalType", default)]
            terminal_type: String,
        }
        #[derive(Deserialize)]
        struct RawConnect {
            data: Data,
        }

        let raw: RawConnect = resp.json().await?;
        Ok(ConnectExReply {
            terminal_instance_guid: raw.data.terminal_instance_guid,
            terminal_type: raw.data.terminal_type,
        })
    }

    pub async fn disconnect(&self, terminal_id: &str, api_key: &str) -> Result<DisconnectReply, Box<dyn std::error::Error>> {
        let url = format!("{}/Disconnect", self.endpoint);
        let resp = self.client.get(&url)
            .header("APIKey", api_key)
            .header("id", terminal_id)
            .header("User-Agent", "RustCopier/1.0.0")
            .send()
            .await?;

        #[derive(Deserialize)]
        struct Data {
            #[serde(rename = "uniqueIdentifier")]
            unique_identifier: String,
            #[serde(rename = "fullLifeTimeSeconds", default)]
            full_life_time_seconds: i32,
        }
        #[derive(Deserialize)]
        struct RawDisc {
            data: Data,
        }

        let raw: RawDisc = resp.json().await?;
        Ok(DisconnectReply {
            unique_identifier: raw.data.unique_identifier,
            full_life_time_seconds: raw.data.full_life_time_seconds,
        })
    }
}
