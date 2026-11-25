pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryIntegrationsGheServerInstallationResponseProviderMetadataAssetsOnlyWorkersWorkerGroupsValue {
    #[serde(default)]
    pub service: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
}

impl QueryIntegrationsGheServerInstallationResponseProviderMetadataAssetsOnlyWorkersWorkerGroupsValue {
    pub fn builder() -> QueryIntegrationsGheServerInstallationResponseProviderMetadataAssetsOnlyWorkersWorkerGroupsValueBuilder {
        <QueryIntegrationsGheServerInstallationResponseProviderMetadataAssetsOnlyWorkersWorkerGroupsValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsGheServerInstallationResponseProviderMetadataAssetsOnlyWorkersWorkerGroupsValueBuilder {
    service: Option<String>,
    environment: Option<String>,
}

impl QueryIntegrationsGheServerInstallationResponseProviderMetadataAssetsOnlyWorkersWorkerGroupsValueBuilder {
    pub fn service(mut self, value: impl Into<String>) -> Self {
        self.service = Some(value.into());
        self
    }

    pub fn environment(mut self, value: impl Into<String>) -> Self {
        self.environment = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryIntegrationsGheServerInstallationResponseProviderMetadataAssetsOnlyWorkersWorkerGroupsValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`service`](QueryIntegrationsGheServerInstallationResponseProviderMetadataAssetsOnlyWorkersWorkerGroupsValueBuilder::service)
    pub fn build(self) -> Result<QueryIntegrationsGheServerInstallationResponseProviderMetadataAssetsOnlyWorkersWorkerGroupsValue, BuildError> {
        Ok(QueryIntegrationsGheServerInstallationResponseProviderMetadataAssetsOnlyWorkersWorkerGroupsValue {
            service: self.service.ok_or_else(|| BuildError::missing_field("service"))?,
            environment: self.environment,
        })
    }
}
