pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryIntegrationsVercelInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItem {
    #[serde(default)]
    pub framework: QueryIntegrationsVercelInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItemFramework,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub path: String,
}

impl QueryIntegrationsVercelInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItem {
    pub fn builder() -> QueryIntegrationsVercelInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItemBuilder {
        <QueryIntegrationsVercelInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsVercelInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItemBuilder {
    framework: Option<QueryIntegrationsVercelInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItemFramework>,
    name: Option<String>,
    path: Option<String>,
}

impl QueryIntegrationsVercelInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItemBuilder {
    pub fn framework(mut self, value: QueryIntegrationsVercelInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItemFramework) -> Self {
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

    /// Consumes the builder and constructs a [`QueryIntegrationsVercelInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`framework`](QueryIntegrationsVercelInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItemBuilder::framework)
    /// - [`name`](QueryIntegrationsVercelInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItemBuilder::name)
    /// - [`path`](QueryIntegrationsVercelInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItemBuilder::path)
    pub fn build(self) -> Result<QueryIntegrationsVercelInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItem, BuildError> {
        Ok(QueryIntegrationsVercelInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItem {
            framework: self.framework.ok_or_else(|| BuildError::missing_field("framework"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            path: self.path.ok_or_else(|| BuildError::missing_field("path"))?,
        })
    }
}
