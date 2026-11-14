pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListForSurfacePrivateRequestArgs {
    /// Surface URL slug, using lowercase letters, numbers, and hyphens.
    #[serde(rename = "surfaceSlug")]
    #[serde(default)]
    pub surface_slug: String,
}

impl ListForSurfacePrivateRequestArgs {
    pub fn builder() -> ListForSurfacePrivateRequestArgsBuilder {
        <ListForSurfacePrivateRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListForSurfacePrivateRequestArgsBuilder {
    surface_slug: Option<String>,
}

impl ListForSurfacePrivateRequestArgsBuilder {
    pub fn surface_slug(mut self, value: impl Into<String>) -> Self {
        self.surface_slug = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListForSurfacePrivateRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`surface_slug`](ListForSurfacePrivateRequestArgsBuilder::surface_slug)
    pub fn build(self) -> Result<ListForSurfacePrivateRequestArgs, BuildError> {
        Ok(ListForSurfacePrivateRequestArgs {
            surface_slug: self.surface_slug.ok_or_else(|| BuildError::missing_field("surface_slug"))?,
        })
    }
}
