pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryIntegrationsCloudflareInstallationResponseProviderMetadataAccountIdRepositoryScansValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apps: Option<Vec<QueryIntegrationsCloudflareInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItem>>,
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "scannedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub scanned_at: Option<DateTime<FixedOffset>>,
    pub status: QueryIntegrationsCloudflareInstallationResponseProviderMetadataAccountIdRepositoryScansValueStatus,
}

impl QueryIntegrationsCloudflareInstallationResponseProviderMetadataAccountIdRepositoryScansValue {
    pub fn builder() -> QueryIntegrationsCloudflareInstallationResponseProviderMetadataAccountIdRepositoryScansValueBuilder {
        <QueryIntegrationsCloudflareInstallationResponseProviderMetadataAccountIdRepositoryScansValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsCloudflareInstallationResponseProviderMetadataAccountIdRepositoryScansValueBuilder {
    apps: Option<Vec<QueryIntegrationsCloudflareInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItem>>,
    error_message: Option<String>,
    scanned_at: Option<DateTime<FixedOffset>>,
    status: Option<QueryIntegrationsCloudflareInstallationResponseProviderMetadataAccountIdRepositoryScansValueStatus>,
}

impl QueryIntegrationsCloudflareInstallationResponseProviderMetadataAccountIdRepositoryScansValueBuilder {
    pub fn apps(mut self, value: Vec<QueryIntegrationsCloudflareInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItem>) -> Self {
        self.apps = Some(value);
        self
    }

    pub fn error_message(mut self, value: impl Into<String>) -> Self {
        self.error_message = Some(value.into());
        self
    }

    pub fn scanned_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.scanned_at = Some(value);
        self
    }

    pub fn status(mut self, value: QueryIntegrationsCloudflareInstallationResponseProviderMetadataAccountIdRepositoryScansValueStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryIntegrationsCloudflareInstallationResponseProviderMetadataAccountIdRepositoryScansValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](QueryIntegrationsCloudflareInstallationResponseProviderMetadataAccountIdRepositoryScansValueBuilder::status)
    pub fn build(self) -> Result<QueryIntegrationsCloudflareInstallationResponseProviderMetadataAccountIdRepositoryScansValue, BuildError> {
        Ok(QueryIntegrationsCloudflareInstallationResponseProviderMetadataAccountIdRepositoryScansValue {
            apps: self.apps,
            error_message: self.error_message,
            scanned_at: self.scanned_at,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
