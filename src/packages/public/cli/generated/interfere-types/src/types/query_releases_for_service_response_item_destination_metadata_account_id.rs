pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Release destination metadata for a Cloudflare Workers deployment.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryReleasesForServiceResponseItemDestinationMetadataAccountId {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
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
    pub provider: QueryReleasesForServiceResponseItemDestinationMetadataAccountIdProvider,
    #[serde(rename = "deploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    #[serde(rename = "trafficPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_percentage: Option<f64>,
    #[serde(rename = "scriptName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_name: Option<String>,
    #[serde(rename = "versionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

impl QueryReleasesForServiceResponseItemDestinationMetadataAccountId {
    pub fn builder() -> QueryReleasesForServiceResponseItemDestinationMetadataAccountIdBuilder {
        <QueryReleasesForServiceResponseItemDestinationMetadataAccountIdBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryReleasesForServiceResponseItemDestinationMetadataAccountIdBuilder {
    account_id: Option<String>,
    deployment_url: Option<String>,
    destination_release_id: Option<String>,
    environment: Option<String>,
    environment_name: Option<String>,
    environment_target: Option<String>,
    provider: Option<QueryReleasesForServiceResponseItemDestinationMetadataAccountIdProvider>,
    deployment_id: Option<String>,
    traffic_percentage: Option<f64>,
    script_name: Option<String>,
    version_id: Option<String>,
}

impl QueryReleasesForServiceResponseItemDestinationMetadataAccountIdBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
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

    pub fn provider(mut self, value: QueryReleasesForServiceResponseItemDestinationMetadataAccountIdProvider) -> Self {
        self.provider = Some(value);
        self
    }

    pub fn deployment_id(mut self, value: impl Into<String>) -> Self {
        self.deployment_id = Some(value.into());
        self
    }

    pub fn traffic_percentage(mut self, value: f64) -> Self {
        self.traffic_percentage = Some(value);
        self
    }

    pub fn script_name(mut self, value: impl Into<String>) -> Self {
        self.script_name = Some(value.into());
        self
    }

    pub fn version_id(mut self, value: impl Into<String>) -> Self {
        self.version_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryReleasesForServiceResponseItemDestinationMetadataAccountId`].
    /// This method will fail if any of the following fields are not set:
    /// - [`provider`](QueryReleasesForServiceResponseItemDestinationMetadataAccountIdBuilder::provider)
    pub fn build(self) -> Result<QueryReleasesForServiceResponseItemDestinationMetadataAccountId, BuildError> {
        Ok(QueryReleasesForServiceResponseItemDestinationMetadataAccountId {
            account_id: self.account_id,
            deployment_url: self.deployment_url,
            destination_release_id: self.destination_release_id,
            environment: self.environment,
            environment_name: self.environment_name,
            environment_target: self.environment_target,
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
            deployment_id: self.deployment_id,
            traffic_percentage: self.traffic_percentage,
            script_name: self.script_name,
            version_id: self.version_id,
        })
    }
}
