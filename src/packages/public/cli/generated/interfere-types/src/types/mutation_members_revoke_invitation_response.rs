pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationMembersRevokeInvitationResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationMembersRevokeInvitationResponse {
    pub fn builder() -> MutationMembersRevokeInvitationResponseBuilder {
        <MutationMembersRevokeInvitationResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationMembersRevokeInvitationResponseBuilder {
    success: Option<bool>,
}

impl MutationMembersRevokeInvitationResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationMembersRevokeInvitationResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationMembersRevokeInvitationResponseBuilder::success)
    pub fn build(self) -> Result<MutationMembersRevokeInvitationResponse, BuildError> {
        Ok(MutationMembersRevokeInvitationResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
