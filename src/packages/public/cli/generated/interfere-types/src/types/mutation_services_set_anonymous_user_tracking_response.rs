pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationServicesSetAnonymousUserTrackingResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationServicesSetAnonymousUserTrackingResponse {
    pub fn builder() -> MutationServicesSetAnonymousUserTrackingResponseBuilder {
        <MutationServicesSetAnonymousUserTrackingResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationServicesSetAnonymousUserTrackingResponseBuilder {
    success: Option<bool>,
}

impl MutationServicesSetAnonymousUserTrackingResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationServicesSetAnonymousUserTrackingResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationServicesSetAnonymousUserTrackingResponseBuilder::success)
    pub fn build(self) -> Result<MutationServicesSetAnonymousUserTrackingResponse, BuildError> {
        Ok(MutationServicesSetAnonymousUserTrackingResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
