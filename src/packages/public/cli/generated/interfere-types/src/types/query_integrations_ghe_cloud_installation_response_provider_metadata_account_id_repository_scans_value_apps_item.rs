pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryIntegrationsGheCloudInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItem {
    #[serde(default)]
    pub framework: QueryIntegrationsGheCloudInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItemFramework,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub path: String,
}

impl QueryIntegrationsGheCloudInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItem {
    pub fn builder() -> QueryIntegrationsGheCloudInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItemBuilder {
        <QueryIntegrationsGheCloudInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsGheCloudInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItemBuilder {
    framework: Option<QueryIntegrationsGheCloudInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItemFramework>,
    name: Option<String>,
    path: Option<String>,
}

impl QueryIntegrationsGheCloudInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItemBuilder {
    pub fn framework(mut self, value: QueryIntegrationsGheCloudInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItemFramework) -> Self {
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

    /// Consumes the builder and constructs a [`QueryIntegrationsGheCloudInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`framework`](QueryIntegrationsGheCloudInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItemBuilder::framework)
    /// - [`name`](QueryIntegrationsGheCloudInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItemBuilder::name)
    /// - [`path`](QueryIntegrationsGheCloudInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItemBuilder::path)
    pub fn build(self) -> Result<QueryIntegrationsGheCloudInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItem, BuildError> {
        Ok(QueryIntegrationsGheCloudInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItem {
            framework: self.framework.ok_or_else(|| BuildError::missing_field("framework"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            path: self.path.ok_or_else(|| BuildError::missing_field("path"))?,
        })
    }
}
