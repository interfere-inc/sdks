pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RevokeInvitationsRequestArgs {
    #[serde(rename = "clerkInvitationId")]
    #[serde(default)]
    pub clerk_invitation_id: String,
}

impl RevokeInvitationsRequestArgs {
    pub fn builder() -> RevokeInvitationsRequestArgsBuilder {
        <RevokeInvitationsRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RevokeInvitationsRequestArgsBuilder {
    clerk_invitation_id: Option<String>,
}

impl RevokeInvitationsRequestArgsBuilder {
    pub fn clerk_invitation_id(mut self, value: impl Into<String>) -> Self {
        self.clerk_invitation_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RevokeInvitationsRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`clerk_invitation_id`](RevokeInvitationsRequestArgsBuilder::clerk_invitation_id)
    pub fn build(self) -> Result<RevokeInvitationsRequestArgs, BuildError> {
        Ok(RevokeInvitationsRequestArgs {
            clerk_invitation_id: self.clerk_invitation_id.ok_or_else(|| BuildError::missing_field("clerk_invitation_id"))?,
        })
    }
}
