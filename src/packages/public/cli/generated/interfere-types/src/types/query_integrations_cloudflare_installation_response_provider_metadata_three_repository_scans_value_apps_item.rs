pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryIntegrationsCloudflareInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItem {
    #[serde(default)]
    pub framework: QueryIntegrationsCloudflareInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItemFramework,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub path: String,
}

impl QueryIntegrationsCloudflareInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItem {
    pub fn builder() -> QueryIntegrationsCloudflareInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItemBuilder {
        <QueryIntegrationsCloudflareInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsCloudflareInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItemBuilder {
    framework: Option<QueryIntegrationsCloudflareInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItemFramework>,
    name: Option<String>,
    path: Option<String>,
}

impl QueryIntegrationsCloudflareInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItemBuilder {
    pub fn framework(mut self, value: QueryIntegrationsCloudflareInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItemFramework) -> Self {
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

    /// Consumes the builder and constructs a [`QueryIntegrationsCloudflareInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`framework`](QueryIntegrationsCloudflareInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItemBuilder::framework)
    /// - [`name`](QueryIntegrationsCloudflareInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItemBuilder::name)
    /// - [`path`](QueryIntegrationsCloudflareInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItemBuilder::path)
    pub fn build(self) -> Result<QueryIntegrationsCloudflareInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItem, BuildError> {
        Ok(QueryIntegrationsCloudflareInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItem {
            framework: self.framework.ok_or_else(|| BuildError::missing_field("framework"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            path: self.path.ok_or_else(|| BuildError::missing_field("path"))?,
        })
    }
}
