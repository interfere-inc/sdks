pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataAccountIdRepositoryScansValueAppsItem {
    #[serde(default)]
    pub framework: QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataAccountIdRepositoryScansValueAppsItemFramework,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub path: String,
}

impl QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataAccountIdRepositoryScansValueAppsItem {
    pub fn builder() -> QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataAccountIdRepositoryScansValueAppsItemBuilder {
        <QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataAccountIdRepositoryScansValueAppsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataAccountIdRepositoryScansValueAppsItemBuilder {
    framework: Option<QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataAccountIdRepositoryScansValueAppsItemFramework>,
    name: Option<String>,
    path: Option<String>,
}

impl QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataAccountIdRepositoryScansValueAppsItemBuilder {
    pub fn framework(mut self, value: QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataAccountIdRepositoryScansValueAppsItemFramework) -> Self {
        self.framework = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn path(mut self, value: impl Into<String>) -> Self {
        self.path = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataAccountIdRepositoryScansValueAppsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`framework`](QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataAccountIdRepositoryScansValueAppsItemBuilder::framework)
    /// - [`name`](QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataAccountIdRepositoryScansValueAppsItemBuilder::name)
    /// - [`path`](QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataAccountIdRepositoryScansValueAppsItemBuilder::path)
    pub fn build(self) -> Result<QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataAccountIdRepositoryScansValueAppsItem, BuildError> {
        Ok(QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataAccountIdRepositoryScansValueAppsItem {
            framework: self.framework.ok_or_else(|| BuildError::missing_field("framework"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            path: self.path.ok_or_else(|| BuildError::missing_field("path"))?,
        })
    }
}
