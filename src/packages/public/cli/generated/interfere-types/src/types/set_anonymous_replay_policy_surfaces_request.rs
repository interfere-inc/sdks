pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SetAnonymousReplayPolicySurfacesRequest {
    pub args: SetAnonymousReplayPolicySurfacesRequestArgs,
}

impl SetAnonymousReplayPolicySurfacesRequest {
    pub fn builder() -> SetAnonymousReplayPolicySurfacesRequestBuilder {
        <SetAnonymousReplayPolicySurfacesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetAnonymousReplayPolicySurfacesRequestBuilder {
    args: Option<SetAnonymousReplayPolicySurfacesRequestArgs>,
}

impl SetAnonymousReplayPolicySurfacesRequestBuilder {
    pub fn args(mut self, value: SetAnonymousReplayPolicySurfacesRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SetAnonymousReplayPolicySurfacesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](SetAnonymousReplayPolicySurfacesRequestBuilder::args)
    pub fn build(self) -> Result<SetAnonymousReplayPolicySurfacesRequest, BuildError> {
        Ok(SetAnonymousReplayPolicySurfacesRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

