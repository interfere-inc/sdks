pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UnlinkSurfaceDestinationMappingIntegrationsRequestArgs {
    /// Surface URL slug, using lowercase letters, numbers, and hyphens.
    #[serde(rename = "surfaceSlug")]
    #[serde(default)]
    pub surface_slug: String,
}

impl UnlinkSurfaceDestinationMappingIntegrationsRequestArgs {
    pub fn builder() -> UnlinkSurfaceDestinationMappingIntegrationsRequestArgsBuilder {
        <UnlinkSurfaceDestinationMappingIntegrationsRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UnlinkSurfaceDestinationMappingIntegrationsRequestArgsBuilder {
    surface_slug: Option<String>,
}

impl UnlinkSurfaceDestinationMappingIntegrationsRequestArgsBuilder {
    pub fn surface_slug(mut self, value: impl Into<String>) -> Self {
        self.surface_slug = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UnlinkSurfaceDestinationMappingIntegrationsRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`surface_slug`](UnlinkSurfaceDestinationMappingIntegrationsRequestArgsBuilder::surface_slug)
    pub fn build(self) -> Result<UnlinkSurfaceDestinationMappingIntegrationsRequestArgs, BuildError> {
        Ok(UnlinkSurfaceDestinationMappingIntegrationsRequestArgs {
            surface_slug: self.surface_slug.ok_or_else(|| BuildError::missing_field("surface_slug"))?,
        })
    }
}
