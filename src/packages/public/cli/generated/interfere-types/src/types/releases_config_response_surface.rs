pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReleasesConfigResponseSurface {
    #[serde(default)]
    pub id: String,
    /// Surface URL slug, using lowercase letters, numbers, and hyphens.
    #[serde(default)]
    pub slug: String,
    #[serde(rename = "environmentFiltering")]
    #[serde(default)]
    pub environment_filtering: bool,
    #[serde(rename = "sourceProvider")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_provider: Option<ConfiguredSourceProvider>,
    #[serde(rename = "destinationProvider")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_provider: Option<ConfiguredDestinationProvider>,
    #[serde(rename = "sdkConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdk_config: Option<ReleasesConfigResponseSurfaceSdkConfig>,
}

impl ReleasesConfigResponseSurface {
    pub fn builder() -> ReleasesConfigResponseSurfaceBuilder {
        <ReleasesConfigResponseSurfaceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReleasesConfigResponseSurfaceBuilder {
    id: Option<String>,
    slug: Option<String>,
    environment_filtering: Option<bool>,
    source_provider: Option<ConfiguredSourceProvider>,
    destination_provider: Option<ConfiguredDestinationProvider>,
    sdk_config: Option<ReleasesConfigResponseSurfaceSdkConfig>,
}

impl ReleasesConfigResponseSurfaceBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn slug(mut self, value: impl Into<String>) -> Self {
        self.slug = Some(value.into());
        self
    }

    pub fn environment_filtering(mut self, value: bool) -> Self {
        self.environment_filtering = Some(value);
        self
    }

    pub fn source_provider(mut self, value: ConfiguredSourceProvider) -> Self {
        self.source_provider = Some(value);
        self
    }

    pub fn destination_provider(mut self, value: ConfiguredDestinationProvider) -> Self {
        self.destination_provider = Some(value);
        self
    }

    pub fn sdk_config(mut self, value: ReleasesConfigResponseSurfaceSdkConfig) -> Self {
        self.sdk_config = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReleasesConfigResponseSurface`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ReleasesConfigResponseSurfaceBuilder::id)
    /// - [`slug`](ReleasesConfigResponseSurfaceBuilder::slug)
    /// - [`environment_filtering`](ReleasesConfigResponseSurfaceBuilder::environment_filtering)
    pub fn build(self) -> Result<ReleasesConfigResponseSurface, BuildError> {
        Ok(ReleasesConfigResponseSurface {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            slug: self.slug.ok_or_else(|| BuildError::missing_field("slug"))?,
            environment_filtering: self.environment_filtering.ok_or_else(|| BuildError::missing_field("environment_filtering"))?,
            source_provider: self.source_provider,
            destination_provider: self.destination_provider,
            sdk_config: self.sdk_config,
        })
    }
}
