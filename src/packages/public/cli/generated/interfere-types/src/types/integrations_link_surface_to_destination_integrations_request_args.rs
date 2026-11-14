pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LinkSurfaceToDestinationIntegrationsRequestArgs {
    /// Identifier of the existing integration installation.
    #[serde(rename = "connectionId")]
    #[serde(default)]
    pub connection_id: String,
    /// Destination project identifier exposed by the integration.
    #[serde(rename = "projectId")]
    #[serde(default)]
    pub project_id: String,
    /// Surface URL slug, using lowercase letters, numbers, and hyphens.
    #[serde(rename = "surfaceSlug")]
    #[serde(default)]
    pub surface_slug: String,
}

impl LinkSurfaceToDestinationIntegrationsRequestArgs {
    pub fn builder() -> LinkSurfaceToDestinationIntegrationsRequestArgsBuilder {
        <LinkSurfaceToDestinationIntegrationsRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LinkSurfaceToDestinationIntegrationsRequestArgsBuilder {
    connection_id: Option<String>,
    project_id: Option<String>,
    surface_slug: Option<String>,
}

impl LinkSurfaceToDestinationIntegrationsRequestArgsBuilder {
    pub fn connection_id(mut self, value: impl Into<String>) -> Self {
        self.connection_id = Some(value.into());
        self
    }

    pub fn project_id(mut self, value: impl Into<String>) -> Self {
        self.project_id = Some(value.into());
        self
    }

    pub fn surface_slug(mut self, value: impl Into<String>) -> Self {
        self.surface_slug = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LinkSurfaceToDestinationIntegrationsRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`connection_id`](LinkSurfaceToDestinationIntegrationsRequestArgsBuilder::connection_id)
    /// - [`project_id`](LinkSurfaceToDestinationIntegrationsRequestArgsBuilder::project_id)
    /// - [`surface_slug`](LinkSurfaceToDestinationIntegrationsRequestArgsBuilder::surface_slug)
    pub fn build(self) -> Result<LinkSurfaceToDestinationIntegrationsRequestArgs, BuildError> {
        Ok(LinkSurfaceToDestinationIntegrationsRequestArgs {
            connection_id: self.connection_id.ok_or_else(|| BuildError::missing_field("connection_id"))?,
            project_id: self.project_id.ok_or_else(|| BuildError::missing_field("project_id"))?,
            surface_slug: self.surface_slug.ok_or_else(|| BuildError::missing_field("surface_slug"))?,
        })
    }
}
