pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationSurfacesSetAnonymousReplayPolicyResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationSurfacesSetAnonymousReplayPolicyResponse {
    pub fn builder() -> MutationSurfacesSetAnonymousReplayPolicyResponseBuilder {
        <MutationSurfacesSetAnonymousReplayPolicyResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationSurfacesSetAnonymousReplayPolicyResponseBuilder {
    success: Option<bool>,
}

impl MutationSurfacesSetAnonymousReplayPolicyResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationSurfacesSetAnonymousReplayPolicyResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationSurfacesSetAnonymousReplayPolicyResponseBuilder::success)
    pub fn build(self) -> Result<MutationSurfacesSetAnonymousReplayPolicyResponse, BuildError> {
        Ok(MutationSurfacesSetAnonymousReplayPolicyResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
