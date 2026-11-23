pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryIntegrationsCloudflareInstallationResponseProviderMetadataThreeRepositoryScansValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apps: Option<Vec<QueryIntegrationsCloudflareInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItem>>,
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "scannedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scanned_at: Option<String>,
    pub status: QueryIntegrationsCloudflareInstallationResponseProviderMetadataThreeRepositoryScansValueStatus,
}

impl QueryIntegrationsCloudflareInstallationResponseProviderMetadataThreeRepositoryScansValue {
    pub fn builder() -> QueryIntegrationsCloudflareInstallationResponseProviderMetadataThreeRepositoryScansValueBuilder {
        <QueryIntegrationsCloudflareInstallationResponseProviderMetadataThreeRepositoryScansValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsCloudflareInstallationResponseProviderMetadataThreeRepositoryScansValueBuilder {
    apps: Option<Vec<QueryIntegrationsCloudflareInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItem>>,
    error_message: Option<String>,
    scanned_at: Option<String>,
    status: Option<QueryIntegrationsCloudflareInstallationResponseProviderMetadataThreeRepositoryScansValueStatus>,
}

impl QueryIntegrationsCloudflareInstallationResponseProviderMetadataThreeRepositoryScansValueBuilder {
    pub fn apps(mut self, value: Vec<QueryIntegrationsCloudflareInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItem>) -> Self {
        self.apps = Some(value);
        self
    }

    pub fn error_message(mut self, value: impl Into<String>) -> Self {
        self.error_message = Some(value.into());
        self
    }

    pub fn scanned_at(mut self, value: impl Into<String>) -> Self {
        self.scanned_at = Some(value.into());
        self
    }

    pub fn status(mut self, value: QueryIntegrationsCloudflareInstallationResponseProviderMetadataThreeRepositoryScansValueStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryIntegrationsCloudflareInstallationResponseProviderMetadataThreeRepositoryScansValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](QueryIntegrationsCloudflareInstallationResponseProviderMetadataThreeRepositoryScansValueBuilder::status)
    pub fn build(self) -> Result<QueryIntegrationsCloudflareInstallationResponseProviderMetadataThreeRepositoryScansValue, BuildError> {
        Ok(QueryIntegrationsCloudflareInstallationResponseProviderMetadataThreeRepositoryScansValue {
            apps: self.apps,
            error_message: self.error_message,
            scanned_at: self.scanned_at,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
