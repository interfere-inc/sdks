pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UnlinkSurfaceMappingIntegrationsRequestArgs {
    /// Surface URL slug, using lowercase letters, numbers, and hyphens.
    #[serde(rename = "surfaceSlug")]
    #[serde(default)]
    pub surface_slug: String,
}

impl UnlinkSurfaceMappingIntegrationsRequestArgs {
    pub fn builder() -> UnlinkSurfaceMappingIntegrationsRequestArgsBuilder {
        <UnlinkSurfaceMappingIntegrationsRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UnlinkSurfaceMappingIntegrationsRequestArgsBuilder {
    surface_slug: Option<String>,
}

impl UnlinkSurfaceMappingIntegrationsRequestArgsBuilder {
    pub fn surface_slug(mut self, value: impl Into<String>) -> Self {
        self.surface_slug = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UnlinkSurfaceMappingIntegrationsRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`surface_slug`](UnlinkSurfaceMappingIntegrationsRequestArgsBuilder::surface_slug)
    pub fn build(self) -> Result<UnlinkSurfaceMappingIntegrationsRequestArgs, BuildError> {
        Ok(UnlinkSurfaceMappingIntegrationsRequestArgs {
            surface_slug: self.surface_slug.ok_or_else(|| BuildError::missing_field("surface_slug"))?,
        })
    }
}
