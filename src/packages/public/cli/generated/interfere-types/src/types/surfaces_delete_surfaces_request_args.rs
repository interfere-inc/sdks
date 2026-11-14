pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteSurfacesRequestArgs {
    /// Surface URL slug, using lowercase letters, numbers, and hyphens.
    #[serde(rename = "surfaceSlug")]
    #[serde(default)]
    pub surface_slug: String,
}

impl DeleteSurfacesRequestArgs {
    pub fn builder() -> DeleteSurfacesRequestArgsBuilder {
        <DeleteSurfacesRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteSurfacesRequestArgsBuilder {
    surface_slug: Option<String>,
}

impl DeleteSurfacesRequestArgsBuilder {
    pub fn surface_slug(mut self, value: impl Into<String>) -> Self {
        self.surface_slug = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteSurfacesRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`surface_slug`](DeleteSurfacesRequestArgsBuilder::surface_slug)
    pub fn build(self) -> Result<DeleteSurfacesRequestArgs, BuildError> {
        Ok(DeleteSurfacesRequestArgs {
            surface_slug: self.surface_slug.ok_or_else(|| BuildError::missing_field("surface_slug"))?,
        })
    }
}
