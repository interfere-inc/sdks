pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListForSurfacePublicRequestArgs {
    /// Surface URL slug, using lowercase letters, numbers, and hyphens.
    #[serde(rename = "surfaceSlug")]
    #[serde(default)]
    pub surface_slug: String,
}

impl ListForSurfacePublicRequestArgs {
    pub fn builder() -> ListForSurfacePublicRequestArgsBuilder {
        <ListForSurfacePublicRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListForSurfacePublicRequestArgsBuilder {
    surface_slug: Option<String>,
}

impl ListForSurfacePublicRequestArgsBuilder {
    pub fn surface_slug(mut self, value: impl Into<String>) -> Self {
        self.surface_slug = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListForSurfacePublicRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`surface_slug`](ListForSurfacePublicRequestArgsBuilder::surface_slug)
    pub fn build(self) -> Result<ListForSurfacePublicRequestArgs, BuildError> {
        Ok(ListForSurfacePublicRequestArgs {
            surface_slug: self.surface_slug.ok_or_else(|| BuildError::missing_field("surface_slug"))?,
        })
    }
}
