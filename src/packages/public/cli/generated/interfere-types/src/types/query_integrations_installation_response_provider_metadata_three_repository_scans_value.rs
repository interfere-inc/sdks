pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryIntegrationsInstallationResponseProviderMetadataThreeRepositoryScansValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apps: Option<Vec<QueryIntegrationsInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directories: Option<Vec<String>>,
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "scannedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scanned_at: Option<String>,
    pub status: QueryIntegrationsInstallationResponseProviderMetadataThreeRepositoryScansValueStatus,
}

impl QueryIntegrationsInstallationResponseProviderMetadataThreeRepositoryScansValue {
    pub fn builder() -> QueryIntegrationsInstallationResponseProviderMetadataThreeRepositoryScansValueBuilder {
        <QueryIntegrationsInstallationResponseProviderMetadataThreeRepositoryScansValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsInstallationResponseProviderMetadataThreeRepositoryScansValueBuilder {
    apps: Option<Vec<QueryIntegrationsInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItem>>,
    directories: Option<Vec<String>>,
    error_message: Option<String>,
    scanned_at: Option<String>,
    status: Option<QueryIntegrationsInstallationResponseProviderMetadataThreeRepositoryScansValueStatus>,
}

impl QueryIntegrationsInstallationResponseProviderMetadataThreeRepositoryScansValueBuilder {
    pub fn apps(mut self, value: Vec<QueryIntegrationsInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItem>) -> Self {
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

    pub fn status(mut self, value: QueryIntegrationsInstallationResponseProviderMetadataThreeRepositoryScansValueStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryIntegrationsInstallationResponseProviderMetadataThreeRepositoryScansValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](QueryIntegrationsInstallationResponseProviderMetadataThreeRepositoryScansValueBuilder::status)
    pub fn build(self) -> Result<QueryIntegrationsInstallationResponseProviderMetadataThreeRepositoryScansValue, BuildError> {
        Ok(QueryIntegrationsInstallationResponseProviderMetadataThreeRepositoryScansValue {
            apps: self.apps,
            directories: self.directories,
            error_message: self.error_message,
            scanned_at: self.scanned_at,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
