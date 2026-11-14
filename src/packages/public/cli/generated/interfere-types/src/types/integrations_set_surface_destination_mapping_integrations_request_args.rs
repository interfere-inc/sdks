pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SetSurfaceDestinationMappingIntegrationsRequestArgs {
    #[serde(default)]
    pub connected: bool,
    #[serde(rename = "connectionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
    #[serde(rename = "mappingId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mapping_id: Option<String>,
    /// Surface URL slug, using lowercase letters, numbers, and hyphens.
    #[serde(rename = "surfaceSlug")]
    #[serde(default)]
    pub surface_slug: String,
}

impl SetSurfaceDestinationMappingIntegrationsRequestArgs {
    pub fn builder() -> SetSurfaceDestinationMappingIntegrationsRequestArgsBuilder {
        <SetSurfaceDestinationMappingIntegrationsRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetSurfaceDestinationMappingIntegrationsRequestArgsBuilder {
    connected: Option<bool>,
    connection_id: Option<String>,
    mapping_id: Option<String>,
    surface_slug: Option<String>,
}

impl SetSurfaceDestinationMappingIntegrationsRequestArgsBuilder {
    pub fn connected(mut self, value: bool) -> Self {
        self.connected = Some(value);
        self
    }

    pub fn connection_id(mut self, value: impl Into<String>) -> Self {
        self.connection_id = Some(value.into());
        self
    }

    pub fn mapping_id(mut self, value: impl Into<String>) -> Self {
        self.mapping_id = Some(value.into());
        self
    }

    pub fn surface_slug(mut self, value: impl Into<String>) -> Self {
        self.surface_slug = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SetSurfaceDestinationMappingIntegrationsRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`connected`](SetSurfaceDestinationMappingIntegrationsRequestArgsBuilder::connected)
    /// - [`surface_slug`](SetSurfaceDestinationMappingIntegrationsRequestArgsBuilder::surface_slug)
    pub fn build(self) -> Result<SetSurfaceDestinationMappingIntegrationsRequestArgs, BuildError> {
        Ok(SetSurfaceDestinationMappingIntegrationsRequestArgs {
            connected: self.connected.ok_or_else(|| BuildError::missing_field("connected"))?,
            connection_id: self.connection_id,
            mapping_id: self.mapping_id,
            surface_slug: self.surface_slug.ok_or_else(|| BuildError::missing_field("surface_slug"))?,
        })
    }
}
