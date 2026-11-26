use crate::{ApiError, ClientConfig, HttpClient};

pub mod environments;
pub use environments::EnvironmentsClient2;
pub struct SurfacesClient {
    pub http_client: HttpClient,
    pub environments: EnvironmentsClient2,
}

impl SurfacesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            environments: EnvironmentsClient2::new(config.clone())?,
        })
    }
}
