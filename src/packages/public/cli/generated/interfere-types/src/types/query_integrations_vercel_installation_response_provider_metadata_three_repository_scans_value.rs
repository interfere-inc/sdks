pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryIntegrationsVercelInstallationResponseProviderMetadataThreeRepositoryScansValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apps: Option<Vec<QueryIntegrationsVercelInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItem>>,
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "scannedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scanned_at: Option<String>,
    pub status: QueryIntegrationsVercelInstallationResponseProviderMetadataThreeRepositoryScansValueStatus,
}

impl QueryIntegrationsVercelInstallationResponseProviderMetadataThreeRepositoryScansValue {
    pub fn builder() -> QueryIntegrationsVercelInstallationResponseProviderMetadataThreeRepositoryScansValueBuilder {
        <QueryIntegrationsVercelInstallationResponseProviderMetadataThreeRepositoryScansValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsVercelInstallationResponseProviderMetadataThreeRepositoryScansValueBuilder {
    apps: Option<Vec<QueryIntegrationsVercelInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItem>>,
    error_message: Option<String>,
    scanned_at: Option<String>,
    status: Option<QueryIntegrationsVercelInstallationResponseProviderMetadataThreeRepositoryScansValueStatus>,
}

impl QueryIntegrationsVercelInstallationResponseProviderMetadataThreeRepositoryScansValueBuilder {
    pub fn apps(mut self, value: Vec<QueryIntegrationsVercelInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItem>) -> Self {
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

    pub fn status(mut self, value: QueryIntegrationsVercelInstallationResponseProviderMetadataThreeRepositoryScansValueStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryIntegrationsVercelInstallationResponseProviderMetadataThreeRepositoryScansValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](QueryIntegrationsVercelInstallationResponseProviderMetadataThreeRepositoryScansValueBuilder::status)
    pub fn build(self) -> Result<QueryIntegrationsVercelInstallationResponseProviderMetadataThreeRepositoryScansValue, BuildError> {
        Ok(QueryIntegrationsVercelInstallationResponseProviderMetadataThreeRepositoryScansValue {
            apps: self.apps,
            error_message: self.error_message,
            scanned_at: self.scanned_at,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
