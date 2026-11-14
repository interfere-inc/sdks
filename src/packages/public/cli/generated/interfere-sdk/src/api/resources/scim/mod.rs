use crate::{ApiError, ClientConfig, HttpClient};

pub mod directories;
pub use directories::DirectoriesClient;
pub mod groups;
pub use groups::GroupsClient;
pub struct ScimClient {
    pub http_client: HttpClient,
    pub directories: DirectoriesClient,
    pub groups: GroupsClient,
}

impl ScimClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            directories: DirectoriesClient::new(config.clone())?,
            groups: GroupsClient::new(config.clone())?,
        })
    }
}
