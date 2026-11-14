pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryIntegrationsVercelInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItem {
    #[serde(default)]
    pub framework: QueryIntegrationsVercelInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItemFramework,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub path: String,
}

impl QueryIntegrationsVercelInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItem {
    pub fn builder() -> QueryIntegrationsVercelInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItemBuilder {
        <QueryIntegrationsVercelInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsVercelInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItemBuilder {
    framework: Option<QueryIntegrationsVercelInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItemFramework>,
    name: Option<String>,
    path: Option<String>,
}

impl QueryIntegrationsVercelInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItemBuilder {
    pub fn framework(mut self, value: QueryIntegrationsVercelInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItemFramework) -> Self {
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

    /// Consumes the builder and constructs a [`QueryIntegrationsVercelInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`framework`](QueryIntegrationsVercelInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItemBuilder::framework)
    /// - [`name`](QueryIntegrationsVercelInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItemBuilder::name)
    /// - [`path`](QueryIntegrationsVercelInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItemBuilder::path)
    pub fn build(self) -> Result<QueryIntegrationsVercelInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItem, BuildError> {
        Ok(QueryIntegrationsVercelInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItem {
            framework: self.framework.ok_or_else(|| BuildError::missing_field("framework"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            path: self.path.ok_or_else(|| BuildError::missing_field("path"))?,
        })
    }
}
