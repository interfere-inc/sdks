use crate::{ApiError, ClientConfig, HttpClient};

pub mod messages;
pub use messages::MessagesClient;
pub mod threads;
pub use threads::ThreadsClient;
pub struct SupportClient {
    pub http_client: HttpClient,
    pub messages: MessagesClient,
    pub threads: ThreadsClient,
}

impl SupportClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            messages: MessagesClient::new(config.clone())?,
            threads: ThreadsClient::new(config.clone())?,
        })
    }
}
