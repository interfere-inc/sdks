pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationServicesSetAnonymousReplayPolicyResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationServicesSetAnonymousReplayPolicyResponse {
    pub fn builder() -> MutationServicesSetAnonymousReplayPolicyResponseBuilder {
        <MutationServicesSetAnonymousReplayPolicyResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationServicesSetAnonymousReplayPolicyResponseBuilder {
    success: Option<bool>,
}

impl MutationServicesSetAnonymousReplayPolicyResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationServicesSetAnonymousReplayPolicyResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationServicesSetAnonymousReplayPolicyResponseBuilder::success)
    pub fn build(self) -> Result<MutationServicesSetAnonymousReplayPolicyResponse, BuildError> {
        Ok(MutationServicesSetAnonymousReplayPolicyResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
