pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryIntegrationsCloudflareInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItemFramework {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
}

impl QueryIntegrationsCloudflareInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItemFramework {
    pub fn builder() -> QueryIntegrationsCloudflareInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItemFrameworkBuilder {
        <QueryIntegrationsCloudflareInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItemFrameworkBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsCloudflareInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItemFrameworkBuilder {
    id: Option<String>,
    name: Option<String>,
}

impl QueryIntegrationsCloudflareInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItemFrameworkBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryIntegrationsCloudflareInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItemFramework`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](QueryIntegrationsCloudflareInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItemFrameworkBuilder::id)
    /// - [`name`](QueryIntegrationsCloudflareInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItemFrameworkBuilder::name)
    pub fn build(self) -> Result<QueryIntegrationsCloudflareInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItemFramework, BuildError> {
        Ok(QueryIntegrationsCloudflareInstallationResponseProviderMetadataTwoRepositoryScansValueAppsItemFramework {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
