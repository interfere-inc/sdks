pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConnectionProviderMetadataAccountIdRepositoryScansValueAppsItem {
    #[serde(default)]
    pub framework: ConnectionProviderMetadataAccountIdRepositoryScansValueAppsItemFramework,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub path: String,
}

impl ConnectionProviderMetadataAccountIdRepositoryScansValueAppsItem {
    pub fn builder() -> ConnectionProviderMetadataAccountIdRepositoryScansValueAppsItemBuilder {
        <ConnectionProviderMetadataAccountIdRepositoryScansValueAppsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectionProviderMetadataAccountIdRepositoryScansValueAppsItemBuilder {
    framework: Option<ConnectionProviderMetadataAccountIdRepositoryScansValueAppsItemFramework>,
    name: Option<String>,
    path: Option<String>,
}

impl ConnectionProviderMetadataAccountIdRepositoryScansValueAppsItemBuilder {
    pub fn framework(mut self, value: ConnectionProviderMetadataAccountIdRepositoryScansValueAppsItemFramework) -> Self {
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

    /// Consumes the builder and constructs a [`ConnectionProviderMetadataAccountIdRepositoryScansValueAppsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`framework`](ConnectionProviderMetadataAccountIdRepositoryScansValueAppsItemBuilder::framework)
    /// - [`name`](ConnectionProviderMetadataAccountIdRepositoryScansValueAppsItemBuilder::name)
    /// - [`path`](ConnectionProviderMetadataAccountIdRepositoryScansValueAppsItemBuilder::path)
    pub fn build(self) -> Result<ConnectionProviderMetadataAccountIdRepositoryScansValueAppsItem, BuildError> {
        Ok(ConnectionProviderMetadataAccountIdRepositoryScansValueAppsItem {
            framework: self.framework.ok_or_else(|| BuildError::missing_field("framework"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            path: self.path.ok_or_else(|| BuildError::missing_field("path"))?,
        })
    }
}
