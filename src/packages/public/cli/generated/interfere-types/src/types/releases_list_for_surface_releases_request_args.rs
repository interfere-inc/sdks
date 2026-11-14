pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListForSurfaceReleasesRequestArgs {
    /// Surface URL slug, using lowercase letters, numbers, and hyphens.
    #[serde(rename = "surfaceSlug")]
    #[serde(default)]
    pub surface_slug: String,
}

impl ListForSurfaceReleasesRequestArgs {
    pub fn builder() -> ListForSurfaceReleasesRequestArgsBuilder {
        <ListForSurfaceReleasesRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListForSurfaceReleasesRequestArgsBuilder {
    surface_slug: Option<String>,
}

impl ListForSurfaceReleasesRequestArgsBuilder {
    pub fn surface_slug(mut self, value: impl Into<String>) -> Self {
        self.surface_slug = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListForSurfaceReleasesRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`surface_slug`](ListForSurfaceReleasesRequestArgsBuilder::surface_slug)
    pub fn build(self) -> Result<ListForSurfaceReleasesRequestArgs, BuildError> {
        Ok(ListForSurfaceReleasesRequestArgs {
            surface_slug: self.surface_slug.ok_or_else(|| BuildError::missing_field("surface_slug"))?,
        })
    }
}
