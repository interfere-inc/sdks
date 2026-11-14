pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationMembersResendInvitationResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationMembersResendInvitationResponse {
    pub fn builder() -> MutationMembersResendInvitationResponseBuilder {
        <MutationMembersResendInvitationResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationMembersResendInvitationResponseBuilder {
    success: Option<bool>,
}

impl MutationMembersResendInvitationResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationMembersResendInvitationResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationMembersResendInvitationResponseBuilder::success)
    pub fn build(self) -> Result<MutationMembersResendInvitationResponse, BuildError> {
        Ok(MutationMembersResendInvitationResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
