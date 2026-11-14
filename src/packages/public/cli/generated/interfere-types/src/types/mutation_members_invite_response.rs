pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationMembersInviteResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationMembersInviteResponse {
    pub fn builder() -> MutationMembersInviteResponseBuilder {
        <MutationMembersInviteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationMembersInviteResponseBuilder {
    success: Option<bool>,
}

impl MutationMembersInviteResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationMembersInviteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationMembersInviteResponseBuilder::success)
    pub fn build(self) -> Result<MutationMembersInviteResponse, BuildError> {
        Ok(MutationMembersInviteResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
