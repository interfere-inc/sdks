pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryIntegrationsSlackInstallationResponseProviderMetadataTwoRepositoryScansValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apps: Option<Vec<QueryIntegrationsSlackInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItem>>,
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "scannedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scanned_at: Option<String>,
    pub status: QueryIntegrationsSlackInstallationResponseProviderMetadataTwoRepositoryScansValueStatus,
}

impl QueryIntegrationsSlackInstallationResponseProviderMetadataTwoRepositoryScansValue {
    pub fn builder() -> QueryIntegrationsSlackInstallationResponseProviderMetadataTwoRepositoryScansValueBuilder {
        <QueryIntegrationsSlackInstallationResponseProviderMetadataTwoRepositoryScansValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsSlackInstallationResponseProviderMetadataTwoRepositoryScansValueBuilder {
    apps: Option<Vec<QueryIntegrationsSlackInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItem>>,
    error_message: Option<String>,
    scanned_at: Option<String>,
    status: Option<QueryIntegrationsSlackInstallationResponseProviderMetadataTwoRepositoryScansValueStatus>,
}

impl QueryIntegrationsSlackInstallationResponseProviderMetadataTwoRepositoryScansValueBuilder {
    pub fn apps(mut self, value: Vec<QueryIntegrationsSlackInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItem>) -> Self {
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

    pub fn status(mut self, value: QueryIntegrationsSlackInstallationResponseProviderMetadataTwoRepositoryScansValueStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryIntegrationsSlackInstallationResponseProviderMetadataTwoRepositoryScansValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](QueryIntegrationsSlackInstallationResponseProviderMetadataTwoRepositoryScansValueBuilder::status)
    pub fn build(self) -> Result<QueryIntegrationsSlackInstallationResponseProviderMetadataTwoRepositoryScansValue, BuildError> {
        Ok(QueryIntegrationsSlackInstallationResponseProviderMetadataTwoRepositoryScansValue {
            apps: self.apps,
            error_message: self.error_message,
            scanned_at: self.scanned_at,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
