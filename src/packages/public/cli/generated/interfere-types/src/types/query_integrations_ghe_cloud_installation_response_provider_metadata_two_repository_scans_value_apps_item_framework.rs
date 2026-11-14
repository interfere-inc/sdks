pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryIntegrationsGheCloudInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItemFramework {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
}

impl QueryIntegrationsGheCloudInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItemFramework {
    pub fn builder() -> QueryIntegrationsGheCloudInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItemFrameworkBuilder {
        <QueryIntegrationsGheCloudInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItemFrameworkBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsGheCloudInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItemFrameworkBuilder {
    id: Option<String>,
    name: Option<String>,
}

impl QueryIntegrationsGheCloudInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItemFrameworkBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryIntegrationsGheCloudInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItemFramework`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](QueryIntegrationsGheCloudInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItemFrameworkBuilder::id)
    /// - [`name`](QueryIntegrationsGheCloudInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItemFrameworkBuilder::name)
    pub fn build(self) -> Result<QueryIntegrationsGheCloudInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItemFramework, BuildError> {
        Ok(QueryIntegrationsGheCloudInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItemFramework {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
