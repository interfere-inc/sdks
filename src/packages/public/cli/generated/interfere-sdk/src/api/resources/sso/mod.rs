use crate::{ApiError, ClientConfig, HttpClient};

pub mod connections;
pub use connections::ConnectionsClient;
pub struct SsoClient {
    pub http_client: HttpClient,
    pub connections: ConnectionsClient,
}

impl SsoClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            connections: ConnectionsClient::new(config.clone())?,
        })
    }
}
