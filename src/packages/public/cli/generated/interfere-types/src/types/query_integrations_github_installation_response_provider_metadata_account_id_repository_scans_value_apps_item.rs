pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryIntegrationsGithubInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItem {
    #[serde(default)]
    pub framework: QueryIntegrationsGithubInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItemFramework,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub path: String,
}

impl QueryIntegrationsGithubInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItem {
    pub fn builder() -> QueryIntegrationsGithubInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItemBuilder {
        <QueryIntegrationsGithubInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsGithubInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItemBuilder {
    framework: Option<QueryIntegrationsGithubInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItemFramework>,
    name: Option<String>,
    path: Option<String>,
}

impl QueryIntegrationsGithubInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItemBuilder {
    pub fn framework(mut self, value: QueryIntegrationsGithubInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItemFramework) -> Self {
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

    /// Consumes the builder and constructs a [`QueryIntegrationsGithubInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`framework`](QueryIntegrationsGithubInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItemBuilder::framework)
    /// - [`name`](QueryIntegrationsGithubInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItemBuilder::name)
    /// - [`path`](QueryIntegrationsGithubInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItemBuilder::path)
    pub fn build(self) -> Result<QueryIntegrationsGithubInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItem, BuildError> {
        Ok(QueryIntegrationsGithubInstallationResponseProviderMetadataAccountIdRepositoryScansValueAppsItem {
            framework: self.framework.ok_or_else(|| BuildError::missing_field("framework"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            path: self.path.ok_or_else(|| BuildError::missing_field("path"))?,
        })
    }
}
