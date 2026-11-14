pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConnectionProviderMetadataTwoRepositoryScansValueAppsItem {
    #[serde(default)]
    pub framework: ConnectionProviderMetadataTwoRepositoryScansValueAppsItemFramework,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub path: String,
}

impl ConnectionProviderMetadataTwoRepositoryScansValueAppsItem {
    pub fn builder() -> ConnectionProviderMetadataTwoRepositoryScansValueAppsItemBuilder {
        <ConnectionProviderMetadataTwoRepositoryScansValueAppsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectionProviderMetadataTwoRepositoryScansValueAppsItemBuilder {
    framework: Option<ConnectionProviderMetadataTwoRepositoryScansValueAppsItemFramework>,
    name: Option<String>,
    path: Option<String>,
}

impl ConnectionProviderMetadataTwoRepositoryScansValueAppsItemBuilder {
    pub fn framework(mut self, value: ConnectionProviderMetadataTwoRepositoryScansValueAppsItemFramework) -> Self {
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

    /// Consumes the builder and constructs a [`ConnectionProviderMetadataTwoRepositoryScansValueAppsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`framework`](ConnectionProviderMetadataTwoRepositoryScansValueAppsItemBuilder::framework)
    /// - [`name`](ConnectionProviderMetadataTwoRepositoryScansValueAppsItemBuilder::name)
    /// - [`path`](ConnectionProviderMetadataTwoRepositoryScansValueAppsItemBuilder::path)
    pub fn build(self) -> Result<ConnectionProviderMetadataTwoRepositoryScansValueAppsItem, BuildError> {
        Ok(ConnectionProviderMetadataTwoRepositoryScansValueAppsItem {
            framework: self.framework.ok_or_else(|| BuildError::missing_field("framework"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            path: self.path.ok_or_else(|| BuildError::missing_field("path"))?,
        })
    }
}
