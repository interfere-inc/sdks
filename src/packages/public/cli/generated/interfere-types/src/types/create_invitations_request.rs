pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateInvitationsRequest {
    pub args: CreateInvitationsRequestArgs,
}

impl CreateInvitationsRequest {
    pub fn builder() -> CreateInvitationsRequestBuilder {
        <CreateInvitationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateInvitationsRequestBuilder {
    args: Option<CreateInvitationsRequestArgs>,
}

impl CreateInvitationsRequestBuilder {
    pub fn args(mut self, value: CreateInvitationsRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateInvitationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](CreateInvitationsRequestBuilder::args)
    pub fn build(self) -> Result<CreateInvitationsRequest, BuildError> {
        Ok(CreateInvitationsRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

