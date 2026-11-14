pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConnectionProviderMetadataThreeRepositoryScansValueAppsItem {
    #[serde(default)]
    pub framework: ConnectionProviderMetadataThreeRepositoryScansValueAppsItemFramework,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub path: String,
}

impl ConnectionProviderMetadataThreeRepositoryScansValueAppsItem {
    pub fn builder() -> ConnectionProviderMetadataThreeRepositoryScansValueAppsItemBuilder {
        <ConnectionProviderMetadataThreeRepositoryScansValueAppsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectionProviderMetadataThreeRepositoryScansValueAppsItemBuilder {
    framework: Option<ConnectionProviderMetadataThreeRepositoryScansValueAppsItemFramework>,
    name: Option<String>,
    path: Option<String>,
}

impl ConnectionProviderMetadataThreeRepositoryScansValueAppsItemBuilder {
    pub fn framework(mut self, value: ConnectionProviderMetadataThreeRepositoryScansValueAppsItemFramework) -> Self {
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

    /// Consumes the builder and constructs a [`ConnectionProviderMetadataThreeRepositoryScansValueAppsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`framework`](ConnectionProviderMetadataThreeRepositoryScansValueAppsItemBuilder::framework)
    /// - [`name`](ConnectionProviderMetadataThreeRepositoryScansValueAppsItemBuilder::name)
    /// - [`path`](ConnectionProviderMetadataThreeRepositoryScansValueAppsItemBuilder::path)
    pub fn build(self) -> Result<ConnectionProviderMetadataThreeRepositoryScansValueAppsItem, BuildError> {
        Ok(ConnectionProviderMetadataThreeRepositoryScansValueAppsItem {
            framework: self.framework.ok_or_else(|| BuildError::missing_field("framework"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            path: self.path.ok_or_else(|| BuildError::missing_field("path"))?,
        })
    }
}
