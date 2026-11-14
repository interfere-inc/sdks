pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryIntegrationsInstallationsResponseItemProviderMetadataThreeRepositoryScansValueAppsItem {
    #[serde(default)]
    pub framework: QueryIntegrationsInstallationsResponseItemProviderMetadataThreeRepositoryScansValueAppsItemFramework,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub path: String,
}

impl QueryIntegrationsInstallationsResponseItemProviderMetadataThreeRepositoryScansValueAppsItem {
    pub fn builder() -> QueryIntegrationsInstallationsResponseItemProviderMetadataThreeRepositoryScansValueAppsItemBuilder {
        <QueryIntegrationsInstallationsResponseItemProviderMetadataThreeRepositoryScansValueAppsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsInstallationsResponseItemProviderMetadataThreeRepositoryScansValueAppsItemBuilder {
    framework: Option<QueryIntegrationsInstallationsResponseItemProviderMetadataThreeRepositoryScansValueAppsItemFramework>,
    name: Option<String>,
    path: Option<String>,
}

impl QueryIntegrationsInstallationsResponseItemProviderMetadataThreeRepositoryScansValueAppsItemBuilder {
    pub fn framework(mut self, value: QueryIntegrationsInstallationsResponseItemProviderMetadataThreeRepositoryScansValueAppsItemFramework) -> Self {
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

    /// Consumes the builder and constructs a [`QueryIntegrationsInstallationsResponseItemProviderMetadataThreeRepositoryScansValueAppsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`framework`](QueryIntegrationsInstallationsResponseItemProviderMetadataThreeRepositoryScansValueAppsItemBuilder::framework)
    /// - [`name`](QueryIntegrationsInstallationsResponseItemProviderMetadataThreeRepositoryScansValueAppsItemBuilder::name)
    /// - [`path`](QueryIntegrationsInstallationsResponseItemProviderMetadataThreeRepositoryScansValueAppsItemBuilder::path)
    pub fn build(self) -> Result<QueryIntegrationsInstallationsResponseItemProviderMetadataThreeRepositoryScansValueAppsItem, BuildError> {
        Ok(QueryIntegrationsInstallationsResponseItemProviderMetadataThreeRepositoryScansValueAppsItem {
            framework: self.framework.ok_or_else(|| BuildError::missing_field("framework"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            path: self.path.ok_or_else(|| BuildError::missing_field("path"))?,
        })
    }
}
