pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryIntegrationsCloudflareInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItem {
    #[serde(default)]
    pub framework: QueryIntegrationsCloudflareInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItemFramework,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub path: String,
}

impl QueryIntegrationsCloudflareInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItem {
    pub fn builder() -> QueryIntegrationsCloudflareInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItemBuilder {
        <QueryIntegrationsCloudflareInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsCloudflareInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItemBuilder {
    framework: Option<QueryIntegrationsCloudflareInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItemFramework>,
    name: Option<String>,
    path: Option<String>,
}

impl QueryIntegrationsCloudflareInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItemBuilder {
    pub fn framework(mut self, value: QueryIntegrationsCloudflareInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItemFramework) -> Self {
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

    /// Consumes the builder and constructs a [`QueryIntegrationsCloudflareInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`framework`](QueryIntegrationsCloudflareInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItemBuilder::framework)
    /// - [`name`](QueryIntegrationsCloudflareInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItemBuilder::name)
    /// - [`path`](QueryIntegrationsCloudflareInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItemBuilder::path)
    pub fn build(self) -> Result<QueryIntegrationsCloudflareInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItem, BuildError> {
        Ok(QueryIntegrationsCloudflareInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItem {
            framework: self.framework.ok_or_else(|| BuildError::missing_field("framework"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            path: self.path.ok_or_else(|| BuildError::missing_field("path"))?,
        })
    }
}
