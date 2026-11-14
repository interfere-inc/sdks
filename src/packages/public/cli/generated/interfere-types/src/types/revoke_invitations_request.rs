pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RevokeInvitationsRequest {
    #[serde(default)]
    pub args: RevokeInvitationsRequestArgs,
}

impl RevokeInvitationsRequest {
    pub fn builder() -> RevokeInvitationsRequestBuilder {
        <RevokeInvitationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RevokeInvitationsRequestBuilder {
    args: Option<RevokeInvitationsRequestArgs>,
}

impl RevokeInvitationsRequestBuilder {
    pub fn args(mut self, value: RevokeInvitationsRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RevokeInvitationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](RevokeInvitationsRequestBuilder::args)
    pub fn build(self) -> Result<RevokeInvitationsRequest, BuildError> {
        Ok(RevokeInvitationsRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

