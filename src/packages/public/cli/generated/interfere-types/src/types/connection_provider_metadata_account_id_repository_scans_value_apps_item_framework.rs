pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConnectionProviderMetadataAccountIdRepositoryScansValueAppsItemFramework {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
}

impl ConnectionProviderMetadataAccountIdRepositoryScansValueAppsItemFramework {
    pub fn builder() -> ConnectionProviderMetadataAccountIdRepositoryScansValueAppsItemFrameworkBuilder {
        <ConnectionProviderMetadataAccountIdRepositoryScansValueAppsItemFrameworkBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectionProviderMetadataAccountIdRepositoryScansValueAppsItemFrameworkBuilder {
    id: Option<String>,
    name: Option<String>,
}

impl ConnectionProviderMetadataAccountIdRepositoryScansValueAppsItemFrameworkBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConnectionProviderMetadataAccountIdRepositoryScansValueAppsItemFramework`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ConnectionProviderMetadataAccountIdRepositoryScansValueAppsItemFrameworkBuilder::id)
    /// - [`name`](ConnectionProviderMetadataAccountIdRepositoryScansValueAppsItemFrameworkBuilder::name)
    pub fn build(self) -> Result<ConnectionProviderMetadataAccountIdRepositoryScansValueAppsItemFramework, BuildError> {
        Ok(ConnectionProviderMetadataAccountIdRepositoryScansValueAppsItemFramework {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
