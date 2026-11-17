pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Response body returned after creating a release.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateReleaseResponse {
    #[serde(rename = "build")]
    #[serde(default)]
    pub build_value: ReleaseBuildRef,
    #[serde(default)]
    pub destination: ReleaseDestinationRef,
    #[serde(default)]
    pub org: OrgRef,
    #[serde(default)]
    pub source: ReleaseSourceRef,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface: Option<SurfaceRef>,
    /// Signed release token for browser telemetry, baked into the bundle by the build plugin. It names the workspace, service and release, carries no environment and does not expire. Null when the release has no service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

impl CreateReleaseResponse {
    pub fn builder() -> CreateReleaseResponseBuilder {
        <CreateReleaseResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateReleaseResponseBuilder {
    build_value: Option<ReleaseBuildRef>,
    destination: Option<ReleaseDestinationRef>,
    org: Option<OrgRef>,
    source: Option<ReleaseSourceRef>,
    surface: Option<SurfaceRef>,
    token: Option<String>,
}

impl CreateReleaseResponseBuilder {
    pub fn build_value(mut self, value: ReleaseBuildRef) -> Self {
        self.build_value = Some(value);
        self
    }

    pub fn destination(mut self, value: ReleaseDestinationRef) -> Self {
        self.destination = Some(value);
        self
    }

    pub fn org(mut self, value: OrgRef) -> Self {
        self.org = Some(value);
        self
    }

    pub fn source(mut self, value: ReleaseSourceRef) -> Self {
        self.source = Some(value);
        self
    }

    pub fn surface(mut self, value: SurfaceRef) -> Self {
        self.surface = Some(value);
        self
    }

    pub fn token(mut self, value: impl Into<String>) -> Self {
        self.token = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateReleaseResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`build_value`](CreateReleaseResponseBuilder::build_value)
    /// - [`destination`](CreateReleaseResponseBuilder::destination)
    /// - [`org`](CreateReleaseResponseBuilder::org)
    /// - [`source`](CreateReleaseResponseBuilder::source)
    pub fn build(self) -> Result<CreateReleaseResponse, BuildError> {
        Ok(CreateReleaseResponse {
            build_value: self.build_value.ok_or_else(|| BuildError::missing_field("build_value"))?,
            destination: self.destination.ok_or_else(|| BuildError::missing_field("destination"))?,
            org: self.org.ok_or_else(|| BuildError::missing_field("org"))?,
            source: self.source.ok_or_else(|| BuildError::missing_field("source"))?,
            surface: self.surface,
            token: self.token,
        })
    }
}
