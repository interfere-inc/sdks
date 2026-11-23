pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryIntegrationsGheServerInstallationResponseProviderMetadataThreeRepositoryScansValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apps: Option<Vec<QueryIntegrationsGheServerInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItem>>,
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "scannedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub scanned_at: Option<DateTime<FixedOffset>>,
    pub status: QueryIntegrationsGheServerInstallationResponseProviderMetadataThreeRepositoryScansValueStatus,
}

impl QueryIntegrationsGheServerInstallationResponseProviderMetadataThreeRepositoryScansValue {
    pub fn builder() -> QueryIntegrationsGheServerInstallationResponseProviderMetadataThreeRepositoryScansValueBuilder {
        <QueryIntegrationsGheServerInstallationResponseProviderMetadataThreeRepositoryScansValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsGheServerInstallationResponseProviderMetadataThreeRepositoryScansValueBuilder {
    apps: Option<Vec<QueryIntegrationsGheServerInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItem>>,
    error_message: Option<String>,
    scanned_at: Option<DateTime<FixedOffset>>,
    status: Option<QueryIntegrationsGheServerInstallationResponseProviderMetadataThreeRepositoryScansValueStatus>,
}

impl QueryIntegrationsGheServerInstallationResponseProviderMetadataThreeRepositoryScansValueBuilder {
    pub fn apps(mut self, value: Vec<QueryIntegrationsGheServerInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItem>) -> Self {
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

    pub fn status(mut self, value: QueryIntegrationsGheServerInstallationResponseProviderMetadataThreeRepositoryScansValueStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryIntegrationsGheServerInstallationResponseProviderMetadataThreeRepositoryScansValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](QueryIntegrationsGheServerInstallationResponseProviderMetadataThreeRepositoryScansValueBuilder::status)
    pub fn build(self) -> Result<QueryIntegrationsGheServerInstallationResponseProviderMetadataThreeRepositoryScansValue, BuildError> {
        Ok(QueryIntegrationsGheServerInstallationResponseProviderMetadataThreeRepositoryScansValue {
            apps: self.apps,
            error_message: self.error_message,
            scanned_at: self.scanned_at,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
