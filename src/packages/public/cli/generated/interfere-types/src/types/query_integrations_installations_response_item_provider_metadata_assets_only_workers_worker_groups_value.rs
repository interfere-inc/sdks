pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryIntegrationsInstallationsResponseItemProviderMetadataAssetsOnlyWorkersWorkerGroupsValue {
    #[serde(default)]
    pub service: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
}

impl QueryIntegrationsInstallationsResponseItemProviderMetadataAssetsOnlyWorkersWorkerGroupsValue {
    pub fn builder() -> QueryIntegrationsInstallationsResponseItemProviderMetadataAssetsOnlyWorkersWorkerGroupsValueBuilder {
        <QueryIntegrationsInstallationsResponseItemProviderMetadataAssetsOnlyWorkersWorkerGroupsValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsInstallationsResponseItemProviderMetadataAssetsOnlyWorkersWorkerGroupsValueBuilder {
    service: Option<String>,
    environment: Option<String>,
}

impl QueryIntegrationsInstallationsResponseItemProviderMetadataAssetsOnlyWorkersWorkerGroupsValueBuilder {
    pub fn service(mut self, value: impl Into<String>) -> Self {
        self.service = Some(value.into());
        self
    }

    pub fn environment(mut self, value: impl Into<String>) -> Self {
        self.environment = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryIntegrationsInstallationsResponseItemProviderMetadataAssetsOnlyWorkersWorkerGroupsValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`service`](QueryIntegrationsInstallationsResponseItemProviderMetadataAssetsOnlyWorkersWorkerGroupsValueBuilder::service)
    pub fn build(self) -> Result<QueryIntegrationsInstallationsResponseItemProviderMetadataAssetsOnlyWorkersWorkerGroupsValue, BuildError> {
        Ok(QueryIntegrationsInstallationsResponseItemProviderMetadataAssetsOnlyWorkersWorkerGroupsValue {
            service: self.service.ok_or_else(|| BuildError::missing_field("service"))?,
            environment: self.environment,
        })
    }
}
