pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryIntegrationsSlackInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItemFramework {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
}

impl QueryIntegrationsSlackInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItemFramework {
    pub fn builder() -> QueryIntegrationsSlackInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItemFrameworkBuilder {
        <QueryIntegrationsSlackInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItemFrameworkBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsSlackInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItemFrameworkBuilder {
    id: Option<String>,
    name: Option<String>,
}

impl QueryIntegrationsSlackInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItemFrameworkBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryIntegrationsSlackInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItemFramework`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](QueryIntegrationsSlackInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItemFrameworkBuilder::id)
    /// - [`name`](QueryIntegrationsSlackInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItemFrameworkBuilder::name)
    pub fn build(self) -> Result<QueryIntegrationsSlackInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItemFramework, BuildError> {
        Ok(QueryIntegrationsSlackInstallationResponseProviderMetadataThreeRepositoryScansValueAppsItemFramework {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
