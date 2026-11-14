pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Release destination metadata for a Vercel deployment.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryReleasesLatestBySurfaceResponseDestinationMetadataZero {
    #[serde(rename = "deploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    #[serde(rename = "deploymentUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_url: Option<String>,
    #[serde(rename = "destinationReleaseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_release_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    #[serde(rename = "environmentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_name: Option<String>,
    #[serde(rename = "environmentTarget")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_target: Option<String>,
    pub provider: QueryReleasesLatestBySurfaceResponseDestinationMetadataZeroProvider,
}

impl QueryReleasesLatestBySurfaceResponseDestinationMetadataZero {
    pub fn builder() -> QueryReleasesLatestBySurfaceResponseDestinationMetadataZeroBuilder {
        <QueryReleasesLatestBySurfaceResponseDestinationMetadataZeroBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryReleasesLatestBySurfaceResponseDestinationMetadataZeroBuilder {
    deployment_id: Option<String>,
    deployment_url: Option<String>,
    destination_release_id: Option<String>,
    environment: Option<String>,
    environment_name: Option<String>,
    environment_target: Option<String>,
    provider: Option<QueryReleasesLatestBySurfaceResponseDestinationMetadataZeroProvider>,
}

impl QueryReleasesLatestBySurfaceResponseDestinationMetadataZeroBuilder {
    pub fn deployment_id(mut self, value: impl Into<String>) -> Self {
        self.deployment_id = Some(value.into());
        self
    }

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

    pub fn environment_name(mut self, value: impl Into<String>) -> Self {
        self.environment_name = Some(value.into());
        self
    }

    pub fn environment_target(mut self, value: impl Into<String>) -> Self {
        self.environment_target = Some(value.into());
        self
    }

    pub fn provider(mut self, value: QueryReleasesLatestBySurfaceResponseDestinationMetadataZeroProvider) -> Self {
        self.provider = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryReleasesLatestBySurfaceResponseDestinationMetadataZero`].
    /// This method will fail if any of the following fields are not set:
    /// - [`provider`](QueryReleasesLatestBySurfaceResponseDestinationMetadataZeroBuilder::provider)
    pub fn build(self) -> Result<QueryReleasesLatestBySurfaceResponseDestinationMetadataZero, BuildError> {
        Ok(QueryReleasesLatestBySurfaceResponseDestinationMetadataZero {
            deployment_id: self.deployment_id,
            deployment_url: self.deployment_url,
            destination_release_id: self.destination_release_id,
            environment: self.environment,
            environment_name: self.environment_name,
            environment_target: self.environment_target,
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
        })
    }
}
