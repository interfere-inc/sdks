use crate::{ApiError, ClientConfig, HttpClient};

pub mod private;
pub use private::PrivateClient;
pub mod public;
pub use public::PublicClient;
pub struct KeysClient {
    pub http_client: HttpClient,
    pub private: PrivateClient,
    pub public: PublicClient,
}

impl KeysClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            private: PrivateClient::new(config.clone())?,
            public: PublicClient::new(config.clone())?,
        })
    }
}
