pub mod proto {
    tonic::include_proto!("copier");
}

use crate::models::*;
use proto::copier_service_client::CopierServiceClient;
use tonic::transport::{Channel, ClientTlsConfig};
use tonic::Request;

pub struct CopierService {
    user_key: String,
    client: CopierServiceClient<Channel>,
}

impl CopierService {
    pub async fn connect(endpoint: &str, user_key: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let clean = endpoint
            .trim_start_matches("http://")
            .trim_start_matches("https://")
            .trim_end_matches(":443")
            .trim_end_matches('/');
        let uri = format!("https://{}:443", clean);
        let tls = ClientTlsConfig::new().with_native_roots();
        let channel = Channel::from_shared(uri)?
            .tls_config(tls)?
            .connect()
            .await?;
        let client = CopierServiceClient::new(channel);
        Ok(Self {
            user_key: user_key.to_string(),
            client,
        })
    }

    pub async fn start(&self, req: StartRequest) -> Result<StartReply, Box<dyn std::error::Error>> {
        let mut client = self.client.clone();
        let user_key = if req.user_key.is_empty() { self.user_key.clone() } else { req.user_key };
        let manager_key = if req.manager_key.is_empty() { self.user_key.clone() } else { req.manager_key };

        let proto_req = proto::StartRequest {
            user_key,
            manager_key,
            master: Some(proto::Account {
                r#type: req.master.r#type,
                user: req.master.user,
                password: req.master.password,
                server: req.master.server,
                name: req.master.name,
                id: req.master.id,
            }),
            slave: Some(proto::Account {
                r#type: req.slave.r#type,
                user: req.slave.user,
                password: req.slave.password,
                server: req.slave.server,
                name: req.slave.name,
                id: req.slave.id,
            }),
            risk_type: req.risk_type,
            risk_value: req.risk_value,
            fixed_master_balance: req.fixed_master_balance,
            copy_sl: req.copy_sl,
            copy_tp: req.copy_tp,
            copy_pending_orders: req.copy_pending_orders,
            reverse_copy: req.reverse_copy,
        };

        let mut request = Request::new(proto_req);
        request.metadata_mut().insert("authorization", format!("Bearer {}", self.user_key).parse()?);
        request.metadata_mut().insert("x-metarpc-client-sdk", "RustCopier/1.0.0".parse()?);
        request.set_timeout(std::time::Duration::from_secs(180));

        match client.start(request).await {
            Ok(resp) => {
                let inner = resp.into_inner();
                Ok(StartReply {
                    ok: inner.ok,
                    copier_id: inner.copier_id,
                    error: inner.error,
                })
            }
            Err(e) => Ok(StartReply {
                ok: false,
                copier_id: "".to_string(),
                error: e.message().to_string(),
            }),
        }
    }

    pub async fn list(&self) -> Result<ListReply, Box<dyn std::error::Error>> {
        let mut client = self.client.clone();
        let proto_req = proto::ListRequest {
            user_key: self.user_key.clone(),
        };
        let mut request = Request::new(proto_req);
        request.metadata_mut().insert("authorization", format!("Bearer {}", self.user_key).parse()?);
        request.metadata_mut().insert("x-metarpc-client-sdk", "RustCopier/1.0.0".parse()?);

        match client.list(request).await {
            Ok(resp) => {
                let inner = resp.into_inner();
                let copiers = inner
                    .copiers
                    .into_iter()
                    .map(|c| CopierSummary {
                        id: c.id,
                        master_type: c.master_type,
                        master_user: c.master_user,
                        master_server: c.master_server,
                        slave_type: c.slave_type,
                        slave_user: c.slave_user,
                        slave_server: c.slave_server,
                        risk_type: c.risk_type,
                        risk_value: c.risk_value,
                        paused: c.paused,
                        pause_reason: c.pause_reason,
                    })
                    .collect();
                Ok(ListReply {
                    ok: inner.ok,
                    copiers,
                    error: inner.error,
                })
            }
            Err(e) => Ok(ListReply {
                ok: false,
                copiers: vec![],
                error: e.message().to_string(),
            }),
        }
    }

    pub async fn pause(&self, copier_id: &str, paused: bool) -> Result<SimpleReply, Box<dyn std::error::Error>> {
        let mut client = self.client.clone();
        let proto_req = proto::PauseRequest {
            user_key: self.user_key.clone(),
            copier_id: copier_id.to_string(),
            paused,
        };
        let mut request = Request::new(proto_req);
        request.metadata_mut().insert("authorization", format!("Bearer {}", self.user_key).parse()?);
        request.metadata_mut().insert("x-metarpc-client-sdk", "RustCopier/1.0.0".parse()?);

        match client.pause(request).await {
            Ok(resp) => {
                let inner = resp.into_inner();
                Ok(SimpleReply {
                    ok: inner.ok,
                    error: inner.error,
                })
            }
            Err(e) => Ok(SimpleReply {
                ok: false,
                error: e.message().to_string(),
            }),
        }
    }

    pub async fn remove(&self, copier_id: &str) -> Result<SimpleReply, Box<dyn std::error::Error>> {
        let mut client = self.client.clone();
        let proto_req = proto::RemoveRequest {
            user_key: self.user_key.clone(),
            copier_id: copier_id.to_string(),
        };
        let mut request = Request::new(proto_req);
        request.metadata_mut().insert("authorization", format!("Bearer {}", self.user_key).parse()?);
        request.metadata_mut().insert("x-metarpc-client-sdk", "RustCopier/1.0.0".parse()?);

        match client.remove(request).await {
            Ok(resp) => {
                let inner = resp.into_inner();
                Ok(SimpleReply {
                    ok: inner.ok,
                    error: inner.error,
                })
            }
            Err(e) => Ok(SimpleReply {
                ok: false,
                error: e.message().to_string(),
            }),
        }
    }
}
