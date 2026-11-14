pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryIntegrationsSlackInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItem {
    #[serde(default)]
    pub framework: QueryIntegrationsSlackInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItemFramework,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub path: String,
}

impl QueryIntegrationsSlackInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItem {
    pub fn builder() -> QueryIntegrationsSlackInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItemBuilder {
        <QueryIntegrationsSlackInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsSlackInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItemBuilder {
    framework: Option<QueryIntegrationsSlackInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItemFramework>,
    name: Option<String>,
    path: Option<String>,
}

impl QueryIntegrationsSlackInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItemBuilder {
    pub fn framework(mut self, value: QueryIntegrationsSlackInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItemFramework) -> Self {
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

    /// Consumes the builder and constructs a [`QueryIntegrationsSlackInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`framework`](QueryIntegrationsSlackInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItemBuilder::framework)
    /// - [`name`](QueryIntegrationsSlackInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItemBuilder::name)
    /// - [`path`](QueryIntegrationsSlackInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItemBuilder::path)
    pub fn build(self) -> Result<QueryIntegrationsSlackInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItem, BuildError> {
        Ok(QueryIntegrationsSlackInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItem {
            framework: self.framework.ok_or_else(|| BuildError::missing_field("framework"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            path: self.path.ok_or_else(|| BuildError::missing_field("path"))?,
        })
    }
}
