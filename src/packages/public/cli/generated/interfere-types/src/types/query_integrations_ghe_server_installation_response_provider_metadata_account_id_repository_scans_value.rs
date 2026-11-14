pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryIntegrationsGheServerInstallationResponseProviderMetadataAccountIdRepositoryScansValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apps: Option<Vec<QueryIntegrationsGheServerInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItem>>,
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "scannedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scanned_at: Option<String>,
    pub status: QueryIntegrationsGheServerInstallationResponseProviderMetadataAccountIdRepositoryScansValueStatus,
}

impl QueryIntegrationsGheServerInstallationResponseProviderMetadataAccountIdRepositoryScansValue {
    pub fn builder() -> QueryIntegrationsGheServerInstallationResponseProviderMetadataAccountIdRepositoryScansValueBuilder {
        <QueryIntegrationsGheServerInstallationResponseProviderMetadataAccountIdRepositoryScansValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsGheServerInstallationResponseProviderMetadataAccountIdRepositoryScansValueBuilder {
    apps: Option<Vec<QueryIntegrationsGheServerInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItem>>,
    error_message: Option<String>,
    scanned_at: Option<String>,
    status: Option<QueryIntegrationsGheServerInstallationResponseProviderMetadataAccountIdRepositoryScansValueStatus>,
}

impl QueryIntegrationsGheServerInstallationResponseProviderMetadataAccountIdRepositoryScansValueBuilder {
    pub fn apps(mut self, value: Vec<QueryIntegrationsGheServerInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItem>) -> Self {
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

    pub fn status(mut self, value: QueryIntegrationsGheServerInstallationResponseProviderMetadataAccountIdRepositoryScansValueStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryIntegrationsGheServerInstallationResponseProviderMetadataAccountIdRepositoryScansValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](QueryIntegrationsGheServerInstallationResponseProviderMetadataAccountIdRepositoryScansValueBuilder::status)
    pub fn build(self) -> Result<QueryIntegrationsGheServerInstallationResponseProviderMetadataAccountIdRepositoryScansValue, BuildError> {
        Ok(QueryIntegrationsGheServerInstallationResponseProviderMetadataAccountIdRepositoryScansValue {
            apps: self.apps,
            error_message: self.error_message,
            scanned_at: self.scanned_at,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
