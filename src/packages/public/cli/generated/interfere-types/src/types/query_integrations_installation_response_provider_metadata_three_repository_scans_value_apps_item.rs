pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryIntegrationsInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItem {
    #[serde(default)]
    pub framework: QueryIntegrationsInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItemFramework,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub path: String,
}

impl QueryIntegrationsInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItem {
    pub fn builder() -> QueryIntegrationsInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItemBuilder {
        <QueryIntegrationsInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItemBuilder {
    framework: Option<QueryIntegrationsInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItemFramework>,
    name: Option<String>,
    path: Option<String>,
}

impl QueryIntegrationsInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItemBuilder {
    pub fn framework(mut self, value: QueryIntegrationsInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItemFramework) -> Self {
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

    /// Consumes the builder and constructs a [`QueryIntegrationsInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`framework`](QueryIntegrationsInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItemBuilder::framework)
    /// - [`name`](QueryIntegrationsInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItemBuilder::name)
    /// - [`path`](QueryIntegrationsInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItemBuilder::path)
    pub fn build(self) -> Result<QueryIntegrationsInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItem, BuildError> {
        Ok(QueryIntegrationsInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItem {
            framework: self.framework.ok_or_else(|| BuildError::missing_field("framework"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            path: self.path.ok_or_else(|| BuildError::missing_field("path"))?,
        })
    }
}
