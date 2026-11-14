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
    #[serde(default)]
    pub surface: SurfaceRef,
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

    /// Consumes the builder and constructs a [`CreateReleaseResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`build_value`](CreateReleaseResponseBuilder::build_value)
    /// - [`destination`](CreateReleaseResponseBuilder::destination)
    /// - [`org`](CreateReleaseResponseBuilder::org)
    /// - [`source`](CreateReleaseResponseBuilder::source)
    /// - [`surface`](CreateReleaseResponseBuilder::surface)
    pub fn build(self) -> Result<CreateReleaseResponse, BuildError> {
        Ok(CreateReleaseResponse {
            build_value: self.build_value.ok_or_else(|| BuildError::missing_field("build_value"))?,
            destination: self.destination.ok_or_else(|| BuildError::missing_field("destination"))?,
            org: self.org.ok_or_else(|| BuildError::missing_field("org"))?,
            source: self.source.ok_or_else(|| BuildError::missing_field("source"))?,
            surface: self.surface.ok_or_else(|| BuildError::missing_field("surface"))?,
        })
    }
}
