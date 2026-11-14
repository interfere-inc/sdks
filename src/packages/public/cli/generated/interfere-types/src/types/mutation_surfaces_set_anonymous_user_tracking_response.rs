pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationSurfacesSetAnonymousUserTrackingResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationSurfacesSetAnonymousUserTrackingResponse {
    pub fn builder() -> MutationSurfacesSetAnonymousUserTrackingResponseBuilder {
        <MutationSurfacesSetAnonymousUserTrackingResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationSurfacesSetAnonymousUserTrackingResponseBuilder {
    success: Option<bool>,
}

impl MutationSurfacesSetAnonymousUserTrackingResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationSurfacesSetAnonymousUserTrackingResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationSurfacesSetAnonymousUserTrackingResponseBuilder::success)
    pub fn build(self) -> Result<MutationSurfacesSetAnonymousUserTrackingResponse, BuildError> {
        Ok(MutationSurfacesSetAnonymousUserTrackingResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
