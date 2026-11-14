pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SurfaceSdkStatusResponse {
    #[serde(rename = "outdatedSurfaceIds")]
    #[serde(default)]
    pub outdated_surface_ids: Vec<String>,
    #[serde(rename = "unanalysedSurfaceIds")]
    #[serde(default)]
    pub unanalysed_surface_ids: Vec<String>,
}

impl SurfaceSdkStatusResponse {
    pub fn builder() -> SurfaceSdkStatusResponseBuilder {
        <SurfaceSdkStatusResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SurfaceSdkStatusResponseBuilder {
    outdated_surface_ids: Option<Vec<String>>,
    unanalysed_surface_ids: Option<Vec<String>>,
}

impl SurfaceSdkStatusResponseBuilder {
    pub fn outdated_surface_ids(mut self, value: Vec<String>) -> Self {
        self.outdated_surface_ids = Some(value);
        self
    }

    pub fn unanalysed_surface_ids(mut self, value: Vec<String>) -> Self {
        self.unanalysed_surface_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SurfaceSdkStatusResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`outdated_surface_ids`](SurfaceSdkStatusResponseBuilder::outdated_surface_ids)
    /// - [`unanalysed_surface_ids`](SurfaceSdkStatusResponseBuilder::unanalysed_surface_ids)
    pub fn build(self) -> Result<SurfaceSdkStatusResponse, BuildError> {
        Ok(SurfaceSdkStatusResponse {
            outdated_surface_ids: self.outdated_surface_ids.ok_or_else(|| BuildError::missing_field("outdated_surface_ids"))?,
            unanalysed_surface_ids: self.unanalysed_surface_ids.ok_or_else(|| BuildError::missing_field("unanalysed_surface_ids"))?,
        })
    }
}
