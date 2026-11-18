pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Service and organization a build-time API key resolves to. surface is deprecated: read service instead; surface is removed in the next API version. It carries the App with the source provider collapsed to the family the build tooling understands, and is null when a workspace secret key builds for a service that has no App.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReleasesConfigResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface: Option<ReleasesConfigResponseSurface>,
    #[serde(default)]
    pub org: ReleasesConfigResponseOrg,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<ReleaseServiceRef>,
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
    service: Option<ReleaseServiceRef>,
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

    pub fn service(mut self, value: ReleaseServiceRef) -> Self {
        self.service = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReleasesConfigResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`org`](ReleasesConfigResponseBuilder::org)
    pub fn build(self) -> Result<ReleasesConfigResponse, BuildError> {
        Ok(ReleasesConfigResponse {
            surface: self.surface,
            org: self.org.ok_or_else(|| BuildError::missing_field("org"))?,
            service: self.service,
        })
    }
}
