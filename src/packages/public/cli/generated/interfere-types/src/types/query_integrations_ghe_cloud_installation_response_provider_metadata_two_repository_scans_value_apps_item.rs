pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryIntegrationsGheCloudInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItem {
    #[serde(default)]
    pub framework: QueryIntegrationsGheCloudInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItemFramework,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub path: String,
}

impl QueryIntegrationsGheCloudInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItem {
    pub fn builder() -> QueryIntegrationsGheCloudInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItemBuilder {
        <QueryIntegrationsGheCloudInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsGheCloudInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItemBuilder {
    framework: Option<QueryIntegrationsGheCloudInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItemFramework>,
    name: Option<String>,
    path: Option<String>,
}

impl QueryIntegrationsGheCloudInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItemBuilder {
    pub fn framework(mut self, value: QueryIntegrationsGheCloudInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItemFramework) -> Self {
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

    /// Consumes the builder and constructs a [`QueryIntegrationsGheCloudInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`framework`](QueryIntegrationsGheCloudInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItemBuilder::framework)
    /// - [`name`](QueryIntegrationsGheCloudInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItemBuilder::name)
    /// - [`path`](QueryIntegrationsGheCloudInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItemBuilder::path)
    pub fn build(self) -> Result<QueryIntegrationsGheCloudInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItem, BuildError> {
        Ok(QueryIntegrationsGheCloudInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItem {
            framework: self.framework.ok_or_else(|| BuildError::missing_field("framework"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            path: self.path.ok_or_else(|| BuildError::missing_field("path"))?,
        })
    }
}
