pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryIntegrationsByIdResponseProviderMetadataThreeRepositoryScansValueAppsItem {
    #[serde(default)]
    pub framework: QueryIntegrationsByIdResponseProviderMetadataThreeRepositoryScansValueAppsItemFramework,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub path: String,
}

impl QueryIntegrationsByIdResponseProviderMetadataThreeRepositoryScansValueAppsItem {
    pub fn builder() -> QueryIntegrationsByIdResponseProviderMetadataThreeRepositoryScansValueAppsItemBuilder {
        <QueryIntegrationsByIdResponseProviderMetadataThreeRepositoryScansValueAppsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsByIdResponseProviderMetadataThreeRepositoryScansValueAppsItemBuilder {
    framework: Option<QueryIntegrationsByIdResponseProviderMetadataThreeRepositoryScansValueAppsItemFramework>,
    name: Option<String>,
    path: Option<String>,
}

impl QueryIntegrationsByIdResponseProviderMetadataThreeRepositoryScansValueAppsItemBuilder {
    pub fn framework(mut self, value: QueryIntegrationsByIdResponseProviderMetadataThreeRepositoryScansValueAppsItemFramework) -> Self {
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

    /// Consumes the builder and constructs a [`QueryIntegrationsByIdResponseProviderMetadataThreeRepositoryScansValueAppsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`framework`](QueryIntegrationsByIdResponseProviderMetadataThreeRepositoryScansValueAppsItemBuilder::framework)
    /// - [`name`](QueryIntegrationsByIdResponseProviderMetadataThreeRepositoryScansValueAppsItemBuilder::name)
    /// - [`path`](QueryIntegrationsByIdResponseProviderMetadataThreeRepositoryScansValueAppsItemBuilder::path)
    pub fn build(self) -> Result<QueryIntegrationsByIdResponseProviderMetadataThreeRepositoryScansValueAppsItem, BuildError> {
        Ok(QueryIntegrationsByIdResponseProviderMetadataThreeRepositoryScansValueAppsItem {
            framework: self.framework.ok_or_else(|| BuildError::missing_field("framework"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            path: self.path.ok_or_else(|| BuildError::missing_field("path"))?,
        })
    }
}
