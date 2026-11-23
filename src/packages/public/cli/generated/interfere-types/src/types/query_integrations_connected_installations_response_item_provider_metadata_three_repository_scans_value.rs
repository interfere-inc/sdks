pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataThreeRepositoryScansValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apps: Option<Vec<QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataThreeRepositoryScansValueAppsItem>>,
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "scannedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub scanned_at: Option<DateTime<FixedOffset>>,
    pub status: QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataThreeRepositoryScansValueStatus,
}

impl QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataThreeRepositoryScansValue {
    pub fn builder() -> QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataThreeRepositoryScansValueBuilder {
        <QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataThreeRepositoryScansValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataThreeRepositoryScansValueBuilder {
    apps: Option<Vec<QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataThreeRepositoryScansValueAppsItem>>,
    error_message: Option<String>,
    scanned_at: Option<DateTime<FixedOffset>>,
    status: Option<QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataThreeRepositoryScansValueStatus>,
}

impl QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataThreeRepositoryScansValueBuilder {
    pub fn apps(mut self, value: Vec<QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataThreeRepositoryScansValueAppsItem>) -> Self {
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

    pub fn status(mut self, value: QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataThreeRepositoryScansValueStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataThreeRepositoryScansValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataThreeRepositoryScansValueBuilder::status)
    pub fn build(self) -> Result<QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataThreeRepositoryScansValue, BuildError> {
        Ok(QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataThreeRepositoryScansValue {
            apps: self.apps,
            error_message: self.error_message,
            scanned_at: self.scanned_at,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
