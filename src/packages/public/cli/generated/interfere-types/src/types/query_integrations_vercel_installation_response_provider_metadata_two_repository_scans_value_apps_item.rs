pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryIntegrationsVercelInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItem {
    #[serde(default)]
    pub framework: QueryIntegrationsVercelInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItemFramework,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub path: String,
}

impl QueryIntegrationsVercelInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItem {
    pub fn builder() -> QueryIntegrationsVercelInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItemBuilder {
        <QueryIntegrationsVercelInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsVercelInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItemBuilder {
    framework: Option<QueryIntegrationsVercelInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItemFramework>,
    name: Option<String>,
    path: Option<String>,
}

impl QueryIntegrationsVercelInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItemBuilder {
    pub fn framework(mut self, value: QueryIntegrationsVercelInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItemFramework) -> Self {
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

    /// Consumes the builder and constructs a [`QueryIntegrationsVercelInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`framework`](QueryIntegrationsVercelInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItemBuilder::framework)
    /// - [`name`](QueryIntegrationsVercelInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItemBuilder::name)
    /// - [`path`](QueryIntegrationsVercelInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItemBuilder::path)
    pub fn build(self) -> Result<QueryIntegrationsVercelInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItem, BuildError> {
        Ok(QueryIntegrationsVercelInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItem {
            framework: self.framework.ok_or_else(|| BuildError::missing_field("framework"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            path: self.path.ok_or_else(|| BuildError::missing_field("path"))?,
        })
    }
}
