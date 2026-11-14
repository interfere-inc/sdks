pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryIntegrationsGithubInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItemFramework {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
}

impl QueryIntegrationsGithubInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItemFramework {
    pub fn builder() -> QueryIntegrationsGithubInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItemFrameworkBuilder {
        <QueryIntegrationsGithubInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItemFrameworkBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsGithubInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItemFrameworkBuilder {
    id: Option<String>,
    name: Option<String>,
}

impl QueryIntegrationsGithubInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItemFrameworkBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryIntegrationsGithubInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItemFramework`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](QueryIntegrationsGithubInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItemFrameworkBuilder::id)
    /// - [`name`](QueryIntegrationsGithubInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItemFrameworkBuilder::name)
    pub fn build(self) -> Result<QueryIntegrationsGithubInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItemFramework, BuildError> {
        Ok(QueryIntegrationsGithubInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItemFramework {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
