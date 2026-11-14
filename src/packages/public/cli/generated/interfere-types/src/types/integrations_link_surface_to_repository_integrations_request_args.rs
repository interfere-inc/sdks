pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LinkSurfaceToRepositoryIntegrationsRequestArgs {
    /// Identifier of the existing integration installation.
    #[serde(rename = "connectionId")]
    #[serde(default)]
    pub connection_id: String,
    /// Repository identifier exposed by the integration.
    #[serde(rename = "repositoryId")]
    #[serde(default)]
    pub repository_id: String,
    /// Surface URL slug, using lowercase letters, numbers, and hyphens.
    #[serde(rename = "surfaceSlug")]
    #[serde(default)]
    pub surface_slug: String,
    /// Repository-relative working directory. Omit for the repository root.
    #[serde(rename = "workingDirectory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_directory: Option<String>,
}

impl LinkSurfaceToRepositoryIntegrationsRequestArgs {
    pub fn builder() -> LinkSurfaceToRepositoryIntegrationsRequestArgsBuilder {
        <LinkSurfaceToRepositoryIntegrationsRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LinkSurfaceToRepositoryIntegrationsRequestArgsBuilder {
    connection_id: Option<String>,
    repository_id: Option<String>,
    surface_slug: Option<String>,
    working_directory: Option<String>,
}

impl LinkSurfaceToRepositoryIntegrationsRequestArgsBuilder {
    pub fn connection_id(mut self, value: impl Into<String>) -> Self {
        self.connection_id = Some(value.into());
        self
    }

    pub fn repository_id(mut self, value: impl Into<String>) -> Self {
        self.repository_id = Some(value.into());
        self
    }

    pub fn surface_slug(mut self, value: impl Into<String>) -> Self {
        self.surface_slug = Some(value.into());
        self
    }

    pub fn working_directory(mut self, value: impl Into<String>) -> Self {
        self.working_directory = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LinkSurfaceToRepositoryIntegrationsRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`connection_id`](LinkSurfaceToRepositoryIntegrationsRequestArgsBuilder::connection_id)
    /// - [`repository_id`](LinkSurfaceToRepositoryIntegrationsRequestArgsBuilder::repository_id)
    /// - [`surface_slug`](LinkSurfaceToRepositoryIntegrationsRequestArgsBuilder::surface_slug)
    pub fn build(self) -> Result<LinkSurfaceToRepositoryIntegrationsRequestArgs, BuildError> {
        Ok(LinkSurfaceToRepositoryIntegrationsRequestArgs {
            connection_id: self.connection_id.ok_or_else(|| BuildError::missing_field("connection_id"))?,
            repository_id: self.repository_id.ok_or_else(|| BuildError::missing_field("repository_id"))?,
            surface_slug: self.surface_slug.ok_or_else(|| BuildError::missing_field("surface_slug"))?,
            working_directory: self.working_directory,
        })
    }
}
