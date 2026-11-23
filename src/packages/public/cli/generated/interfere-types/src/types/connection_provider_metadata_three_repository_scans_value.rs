pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ConnectionProviderMetadataThreeRepositoryScansValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apps: Option<Vec<ConnectionProviderMetadataThreeRepositoryScansValueAppsItem>>,
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "scannedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scanned_at: Option<String>,
    pub status: ConnectionProviderMetadataThreeRepositoryScansValueStatus,
}

impl ConnectionProviderMetadataThreeRepositoryScansValue {
    pub fn builder() -> ConnectionProviderMetadataThreeRepositoryScansValueBuilder {
        <ConnectionProviderMetadataThreeRepositoryScansValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectionProviderMetadataThreeRepositoryScansValueBuilder {
    apps: Option<Vec<ConnectionProviderMetadataThreeRepositoryScansValueAppsItem>>,
    error_message: Option<String>,
    scanned_at: Option<String>,
    status: Option<ConnectionProviderMetadataThreeRepositoryScansValueStatus>,
}

impl ConnectionProviderMetadataThreeRepositoryScansValueBuilder {
    pub fn apps(mut self, value: Vec<ConnectionProviderMetadataThreeRepositoryScansValueAppsItem>) -> Self {
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

    pub fn status(mut self, value: ConnectionProviderMetadataThreeRepositoryScansValueStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConnectionProviderMetadataThreeRepositoryScansValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](ConnectionProviderMetadataThreeRepositoryScansValueBuilder::status)
    pub fn build(self) -> Result<ConnectionProviderMetadataThreeRepositoryScansValue, BuildError> {
        Ok(ConnectionProviderMetadataThreeRepositoryScansValue {
            apps: self.apps,
            error_message: self.error_message,
            scanned_at: self.scanned_at,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
