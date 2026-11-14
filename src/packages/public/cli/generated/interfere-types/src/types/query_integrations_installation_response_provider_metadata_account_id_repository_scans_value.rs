pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryIntegrationsInstallationResponseProviderMetadataAccountIdRepositoryScansValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apps: Option<Vec<QueryIntegrationsInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItem>>,
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "scannedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scanned_at: Option<String>,
    pub status: QueryIntegrationsInstallationResponseProviderMetadataAccountIdRepositoryScansValueStatus,
}

impl QueryIntegrationsInstallationResponseProviderMetadataAccountIdRepositoryScansValue {
    pub fn builder() -> QueryIntegrationsInstallationResponseProviderMetadataAccountIdRepositoryScansValueBuilder {
        <QueryIntegrationsInstallationResponseProviderMetadataAccountIdRepositoryScansValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsInstallationResponseProviderMetadataAccountIdRepositoryScansValueBuilder {
    apps: Option<Vec<QueryIntegrationsInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItem>>,
    error_message: Option<String>,
    scanned_at: Option<String>,
    status: Option<QueryIntegrationsInstallationResponseProviderMetadataAccountIdRepositoryScansValueStatus>,
}

impl QueryIntegrationsInstallationResponseProviderMetadataAccountIdRepositoryScansValueBuilder {
    pub fn apps(mut self, value: Vec<QueryIntegrationsInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItem>) -> Self {
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

    pub fn status(mut self, value: QueryIntegrationsInstallationResponseProviderMetadataAccountIdRepositoryScansValueStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryIntegrationsInstallationResponseProviderMetadataAccountIdRepositoryScansValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](QueryIntegrationsInstallationResponseProviderMetadataAccountIdRepositoryScansValueBuilder::status)
    pub fn build(self) -> Result<QueryIntegrationsInstallationResponseProviderMetadataAccountIdRepositoryScansValue, BuildError> {
        Ok(QueryIntegrationsInstallationResponseProviderMetadataAccountIdRepositoryScansValue {
            apps: self.apps,
            error_message: self.error_message,
            scanned_at: self.scanned_at,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
