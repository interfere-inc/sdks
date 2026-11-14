pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataTwoRepositoryScansValueAppsItem {
    #[serde(default)]
    pub framework: QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataTwoRepositoryScansValueAppsItemFramework,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub path: String,
}

impl QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataTwoRepositoryScansValueAppsItem {
    pub fn builder() -> QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataTwoRepositoryScansValueAppsItemBuilder {
        <QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataTwoRepositoryScansValueAppsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataTwoRepositoryScansValueAppsItemBuilder {
    framework: Option<QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataTwoRepositoryScansValueAppsItemFramework>,
    name: Option<String>,
    path: Option<String>,
}

impl QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataTwoRepositoryScansValueAppsItemBuilder {
    pub fn framework(mut self, value: QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataTwoRepositoryScansValueAppsItemFramework) -> Self {
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

    /// Consumes the builder and constructs a [`QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataTwoRepositoryScansValueAppsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`framework`](QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataTwoRepositoryScansValueAppsItemBuilder::framework)
    /// - [`name`](QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataTwoRepositoryScansValueAppsItemBuilder::name)
    /// - [`path`](QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataTwoRepositoryScansValueAppsItemBuilder::path)
    pub fn build(self) -> Result<QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataTwoRepositoryScansValueAppsItem, BuildError> {
        Ok(QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataTwoRepositoryScansValueAppsItem {
            framework: self.framework.ok_or_else(|| BuildError::missing_field("framework"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            path: self.path.ok_or_else(|| BuildError::missing_field("path"))?,
        })
    }
}
