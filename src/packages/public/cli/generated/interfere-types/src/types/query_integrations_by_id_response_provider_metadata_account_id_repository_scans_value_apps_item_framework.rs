pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryIntegrationsByIdResponseProviderMetadataAccountIdRepositoryScansValueAppsItemFramework {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
}

impl QueryIntegrationsByIdResponseProviderMetadataAccountIdRepositoryScansValueAppsItemFramework {
    pub fn builder() -> QueryIntegrationsByIdResponseProviderMetadataAccountIdRepositoryScansValueAppsItemFrameworkBuilder {
        <QueryIntegrationsByIdResponseProviderMetadataAccountIdRepositoryScansValueAppsItemFrameworkBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsByIdResponseProviderMetadataAccountIdRepositoryScansValueAppsItemFrameworkBuilder {
    id: Option<String>,
    name: Option<String>,
}

impl QueryIntegrationsByIdResponseProviderMetadataAccountIdRepositoryScansValueAppsItemFrameworkBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryIntegrationsByIdResponseProviderMetadataAccountIdRepositoryScansValueAppsItemFramework`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](QueryIntegrationsByIdResponseProviderMetadataAccountIdRepositoryScansValueAppsItemFrameworkBuilder::id)
    /// - [`name`](QueryIntegrationsByIdResponseProviderMetadataAccountIdRepositoryScansValueAppsItemFrameworkBuilder::name)
    pub fn build(self) -> Result<QueryIntegrationsByIdResponseProviderMetadataAccountIdRepositoryScansValueAppsItemFramework, BuildError> {
        Ok(QueryIntegrationsByIdResponseProviderMetadataAccountIdRepositoryScansValueAppsItemFramework {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
