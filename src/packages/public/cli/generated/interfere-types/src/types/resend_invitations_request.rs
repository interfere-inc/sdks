pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ResendInvitationsRequest {
    pub args: ResendInvitationsRequestArgs,
}

impl ResendInvitationsRequest {
    pub fn builder() -> ResendInvitationsRequestBuilder {
        <ResendInvitationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResendInvitationsRequestBuilder {
    args: Option<ResendInvitationsRequestArgs>,
}

impl ResendInvitationsRequestBuilder {
    pub fn args(mut self, value: ResendInvitationsRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResendInvitationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](ResendInvitationsRequestBuilder::args)
    pub fn build(self) -> Result<ResendInvitationsRequest, BuildError> {
        Ok(ResendInvitationsRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

