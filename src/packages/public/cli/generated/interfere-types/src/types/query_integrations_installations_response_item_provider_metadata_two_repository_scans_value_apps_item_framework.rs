pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryIntegrationsInstallationsResponseItemProviderMetadataTwoRepositoryScansValueAppsItemFramework {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
}

impl QueryIntegrationsInstallationsResponseItemProviderMetadataTwoRepositoryScansValueAppsItemFramework {
    pub fn builder() -> QueryIntegrationsInstallationsResponseItemProviderMetadataTwoRepositoryScansValueAppsItemFrameworkBuilder {
        <QueryIntegrationsInstallationsResponseItemProviderMetadataTwoRepositoryScansValueAppsItemFrameworkBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsInstallationsResponseItemProviderMetadataTwoRepositoryScansValueAppsItemFrameworkBuilder {
    id: Option<String>,
    name: Option<String>,
}

impl QueryIntegrationsInstallationsResponseItemProviderMetadataTwoRepositoryScansValueAppsItemFrameworkBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryIntegrationsInstallationsResponseItemProviderMetadataTwoRepositoryScansValueAppsItemFramework`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](QueryIntegrationsInstallationsResponseItemProviderMetadataTwoRepositoryScansValueAppsItemFrameworkBuilder::id)
    /// - [`name`](QueryIntegrationsInstallationsResponseItemProviderMetadataTwoRepositoryScansValueAppsItemFrameworkBuilder::name)
    pub fn build(self) -> Result<QueryIntegrationsInstallationsResponseItemProviderMetadataTwoRepositoryScansValueAppsItemFramework, BuildError> {
        Ok(QueryIntegrationsInstallationsResponseItemProviderMetadataTwoRepositoryScansValueAppsItemFramework {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
