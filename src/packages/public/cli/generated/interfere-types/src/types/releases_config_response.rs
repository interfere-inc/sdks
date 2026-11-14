pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Surface and organization a build-time API key resolves to, with the source provider collapsed to the family the build tooling understands.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReleasesConfigResponse {
    #[serde(default)]
    pub surface: ReleasesConfigResponseSurface,
    #[serde(default)]
    pub org: ReleasesConfigResponseOrg,
}

impl ReleasesConfigResponse {
    pub fn builder() -> ReleasesConfigResponseBuilder {
        <ReleasesConfigResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReleasesConfigResponseBuilder {
    surface: Option<ReleasesConfigResponseSurface>,
    org: Option<ReleasesConfigResponseOrg>,
}

impl ReleasesConfigResponseBuilder {
    pub fn surface(mut self, value: ReleasesConfigResponseSurface) -> Self {
        self.surface = Some(value);
        self
    }

    pub fn org(mut self, value: ReleasesConfigResponseOrg) -> Self {
        self.org = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReleasesConfigResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`surface`](ReleasesConfigResponseBuilder::surface)
    /// - [`org`](ReleasesConfigResponseBuilder::org)
    pub fn build(self) -> Result<ReleasesConfigResponse, BuildError> {
        Ok(ReleasesConfigResponse {
            surface: self.surface.ok_or_else(|| BuildError::missing_field("surface"))?,
            org: self.org.ok_or_else(|| BuildError::missing_field("org"))?,
        })
    }
}
