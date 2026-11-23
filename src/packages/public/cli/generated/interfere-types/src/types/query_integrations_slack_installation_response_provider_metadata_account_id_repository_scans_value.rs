pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryIntegrationsSlackInstallationResponseProviderMetadataAccountIdRepositoryScansValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apps: Option<Vec<QueryIntegrationsSlackInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItem>>,
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "scannedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scanned_at: Option<String>,
    pub status: QueryIntegrationsSlackInstallationResponseProviderMetadataAccountIdRepositoryScansValueStatus,
}

impl QueryIntegrationsSlackInstallationResponseProviderMetadataAccountIdRepositoryScansValue {
    pub fn builder() -> QueryIntegrationsSlackInstallationResponseProviderMetadataAccountIdRepositoryScansValueBuilder {
        <QueryIntegrationsSlackInstallationResponseProviderMetadataAccountIdRepositoryScansValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsSlackInstallationResponseProviderMetadataAccountIdRepositoryScansValueBuilder {
    apps: Option<Vec<QueryIntegrationsSlackInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItem>>,
    error_message: Option<String>,
    scanned_at: Option<String>,
    status: Option<QueryIntegrationsSlackInstallationResponseProviderMetadataAccountIdRepositoryScansValueStatus>,
}

impl QueryIntegrationsSlackInstallationResponseProviderMetadataAccountIdRepositoryScansValueBuilder {
    pub fn apps(mut self, value: Vec<QueryIntegrationsSlackInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItem>) -> Self {
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

    pub fn status(mut self, value: QueryIntegrationsSlackInstallationResponseProviderMetadataAccountIdRepositoryScansValueStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryIntegrationsSlackInstallationResponseProviderMetadataAccountIdRepositoryScansValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](QueryIntegrationsSlackInstallationResponseProviderMetadataAccountIdRepositoryScansValueBuilder::status)
    pub fn build(self) -> Result<QueryIntegrationsSlackInstallationResponseProviderMetadataAccountIdRepositoryScansValue, BuildError> {
        Ok(QueryIntegrationsSlackInstallationResponseProviderMetadataAccountIdRepositoryScansValue {
            apps: self.apps,
            error_message: self.error_message,
            scanned_at: self.scanned_at,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
