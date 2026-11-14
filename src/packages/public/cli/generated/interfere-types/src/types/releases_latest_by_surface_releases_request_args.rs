pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LatestBySurfaceReleasesRequestArgs {
    /// Surface URL slug, using lowercase letters, numbers, and hyphens.
    #[serde(rename = "surfaceSlug")]
    #[serde(default)]
    pub surface_slug: String,
    #[serde(rename = "primaryEnvironment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_environment: Option<String>,
}

impl LatestBySurfaceReleasesRequestArgs {
    pub fn builder() -> LatestBySurfaceReleasesRequestArgsBuilder {
        <LatestBySurfaceReleasesRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LatestBySurfaceReleasesRequestArgsBuilder {
    surface_slug: Option<String>,
    primary_environment: Option<String>,
}

impl LatestBySurfaceReleasesRequestArgsBuilder {
    pub fn surface_slug(mut self, value: impl Into<String>) -> Self {
        self.surface_slug = Some(value.into());
        self
    }

    pub fn primary_environment(mut self, value: impl Into<String>) -> Self {
        self.primary_environment = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LatestBySurfaceReleasesRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`surface_slug`](LatestBySurfaceReleasesRequestArgsBuilder::surface_slug)
    pub fn build(self) -> Result<LatestBySurfaceReleasesRequestArgs, BuildError> {
        Ok(LatestBySurfaceReleasesRequestArgs {
            surface_slug: self.surface_slug.ok_or_else(|| BuildError::missing_field("surface_slug"))?,
            primary_environment: self.primary_environment,
        })
    }
}
