pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryIntegrationsInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItem {
    #[serde(default)]
    pub framework: QueryIntegrationsInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItemFramework,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub path: String,
}

impl QueryIntegrationsInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItem {
    pub fn builder() -> QueryIntegrationsInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItemBuilder {
        <QueryIntegrationsInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItemBuilder {
    framework: Option<QueryIntegrationsInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItemFramework>,
    name: Option<String>,
    path: Option<String>,
}

impl QueryIntegrationsInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItemBuilder {
    pub fn framework(mut self, value: QueryIntegrationsInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItemFramework) -> Self {
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

    /// Consumes the builder and constructs a [`QueryIntegrationsInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`framework`](QueryIntegrationsInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItemBuilder::framework)
    /// - [`name`](QueryIntegrationsInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItemBuilder::name)
    /// - [`path`](QueryIntegrationsInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItemBuilder::path)
    pub fn build(self) -> Result<QueryIntegrationsInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItem, BuildError> {
        Ok(QueryIntegrationsInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItem {
            framework: self.framework.ok_or_else(|| BuildError::missing_field("framework"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            path: self.path.ok_or_else(|| BuildError::missing_field("path"))?,
        })
    }
}
