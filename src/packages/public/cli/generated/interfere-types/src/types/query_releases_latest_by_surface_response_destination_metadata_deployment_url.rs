pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Release destination metadata from the Interfere CLI.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryReleasesLatestBySurfaceResponseDestinationMetadataDeploymentUrl {
    #[serde(rename = "deploymentUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_url: Option<String>,
    #[serde(rename = "destinationReleaseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_release_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    pub provider: QueryReleasesLatestBySurfaceResponseDestinationMetadataDeploymentUrlProvider,
}

impl QueryReleasesLatestBySurfaceResponseDestinationMetadataDeploymentUrl {
    pub fn builder() -> QueryReleasesLatestBySurfaceResponseDestinationMetadataDeploymentUrlBuilder {
        <QueryReleasesLatestBySurfaceResponseDestinationMetadataDeploymentUrlBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryReleasesLatestBySurfaceResponseDestinationMetadataDeploymentUrlBuilder {
    deployment_url: Option<String>,
    destination_release_id: Option<String>,
    environment: Option<String>,
    provider: Option<QueryReleasesLatestBySurfaceResponseDestinationMetadataDeploymentUrlProvider>,
}

impl QueryReleasesLatestBySurfaceResponseDestinationMetadataDeploymentUrlBuilder {
    pub fn deployment_url(mut self, value: impl Into<String>) -> Self {
        self.deployment_url = Some(value.into());
        self
    }

    pub fn destination_release_id(mut self, value: impl Into<String>) -> Self {
        self.destination_release_id = Some(value.into());
        self
    }

    pub fn environment(mut self, value: impl Into<String>) -> Self {
        self.environment = Some(value.into());
        self
    }

    pub fn provider(mut self, value: QueryReleasesLatestBySurfaceResponseDestinationMetadataDeploymentUrlProvider) -> Self {
        self.provider = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryReleasesLatestBySurfaceResponseDestinationMetadataDeploymentUrl`].
    /// This method will fail if any of the following fields are not set:
    /// - [`provider`](QueryReleasesLatestBySurfaceResponseDestinationMetadataDeploymentUrlBuilder::provider)
    pub fn build(self) -> Result<QueryReleasesLatestBySurfaceResponseDestinationMetadataDeploymentUrl, BuildError> {
        Ok(QueryReleasesLatestBySurfaceResponseDestinationMetadataDeploymentUrl {
            deployment_url: self.deployment_url,
            destination_release_id: self.destination_release_id,
            environment: self.environment,
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
        })
    }
}
