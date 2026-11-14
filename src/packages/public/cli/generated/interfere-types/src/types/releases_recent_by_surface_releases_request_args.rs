pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RecentBySurfaceReleasesRequestArgs {
    /// Surface URL slug, using lowercase letters, numbers, and hyphens.
    #[serde(rename = "surfaceSlug")]
    #[serde(default)]
    pub surface_slug: String,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub limit: f64,
}

impl RecentBySurfaceReleasesRequestArgs {
    pub fn builder() -> RecentBySurfaceReleasesRequestArgsBuilder {
        <RecentBySurfaceReleasesRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecentBySurfaceReleasesRequestArgsBuilder {
    surface_slug: Option<String>,
    limit: Option<f64>,
}

impl RecentBySurfaceReleasesRequestArgsBuilder {
    pub fn surface_slug(mut self, value: impl Into<String>) -> Self {
        self.surface_slug = Some(value.into());
        self
    }

    pub fn limit(mut self, value: f64) -> Self {
        self.limit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RecentBySurfaceReleasesRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`surface_slug`](RecentBySurfaceReleasesRequestArgsBuilder::surface_slug)
    /// - [`limit`](RecentBySurfaceReleasesRequestArgsBuilder::limit)
    pub fn build(self) -> Result<RecentBySurfaceReleasesRequestArgs, BuildError> {
        Ok(RecentBySurfaceReleasesRequestArgs {
            surface_slug: self.surface_slug.ok_or_else(|| BuildError::missing_field("surface_slug"))?,
            limit: self.limit.ok_or_else(|| BuildError::missing_field("limit"))?,
        })
    }
}
