pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryIntegrationsInstallationsResponseItemProviderMetadataTwoRepositoryScansValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apps: Option<Vec<QueryIntegrationsInstallationsResponseItemProviderMetadataTwoRepositoryScansValueAppsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directories: Option<Vec<String>>,
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "scannedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scanned_at: Option<String>,
    pub status: QueryIntegrationsInstallationsResponseItemProviderMetadataTwoRepositoryScansValueStatus,
}

impl QueryIntegrationsInstallationsResponseItemProviderMetadataTwoRepositoryScansValue {
    pub fn builder() -> QueryIntegrationsInstallationsResponseItemProviderMetadataTwoRepositoryScansValueBuilder {
        <QueryIntegrationsInstallationsResponseItemProviderMetadataTwoRepositoryScansValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsInstallationsResponseItemProviderMetadataTwoRepositoryScansValueBuilder {
    apps: Option<Vec<QueryIntegrationsInstallationsResponseItemProviderMetadataTwoRepositoryScansValueAppsItem>>,
    directories: Option<Vec<String>>,
    error_message: Option<String>,
    scanned_at: Option<String>,
    status: Option<QueryIntegrationsInstallationsResponseItemProviderMetadataTwoRepositoryScansValueStatus>,
}

impl QueryIntegrationsInstallationsResponseItemProviderMetadataTwoRepositoryScansValueBuilder {
    pub fn apps(mut self, value: Vec<QueryIntegrationsInstallationsResponseItemProviderMetadataTwoRepositoryScansValueAppsItem>) -> Self {
        self.apps = Some(value);
        self
    }

    pub fn directories(mut self, value: Vec<String>) -> Self {
        self.directories = Some(value);
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

    pub fn status(mut self, value: QueryIntegrationsInstallationsResponseItemProviderMetadataTwoRepositoryScansValueStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryIntegrationsInstallationsResponseItemProviderMetadataTwoRepositoryScansValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](QueryIntegrationsInstallationsResponseItemProviderMetadataTwoRepositoryScansValueBuilder::status)
    pub fn build(self) -> Result<QueryIntegrationsInstallationsResponseItemProviderMetadataTwoRepositoryScansValue, BuildError> {
        Ok(QueryIntegrationsInstallationsResponseItemProviderMetadataTwoRepositoryScansValue {
            apps: self.apps,
            directories: self.directories,
            error_message: self.error_message,
            scanned_at: self.scanned_at,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
