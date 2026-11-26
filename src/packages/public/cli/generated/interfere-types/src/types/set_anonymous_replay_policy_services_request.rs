pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SetAnonymousReplayPolicyServicesRequest {
    pub args: SetAnonymousReplayPolicyServicesRequestArgs,
}

impl SetAnonymousReplayPolicyServicesRequest {
    pub fn builder() -> SetAnonymousReplayPolicyServicesRequestBuilder {
        <SetAnonymousReplayPolicyServicesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetAnonymousReplayPolicyServicesRequestBuilder {
    args: Option<SetAnonymousReplayPolicyServicesRequestArgs>,
}

impl SetAnonymousReplayPolicyServicesRequestBuilder {
    pub fn args(mut self, value: SetAnonymousReplayPolicyServicesRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SetAnonymousReplayPolicyServicesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](SetAnonymousReplayPolicyServicesRequestBuilder::args)
    pub fn build(self) -> Result<SetAnonymousReplayPolicyServicesRequest, BuildError> {
        Ok(SetAnonymousReplayPolicyServicesRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

