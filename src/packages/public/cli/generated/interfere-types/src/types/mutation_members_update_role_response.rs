pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationMembersUpdateRoleResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationMembersUpdateRoleResponse {
    pub fn builder() -> MutationMembersUpdateRoleResponseBuilder {
        <MutationMembersUpdateRoleResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationMembersUpdateRoleResponseBuilder {
    success: Option<bool>,
}

impl MutationMembersUpdateRoleResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationMembersUpdateRoleResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationMembersUpdateRoleResponseBuilder::success)
    pub fn build(self) -> Result<MutationMembersUpdateRoleResponse, BuildError> {
        Ok(MutationMembersUpdateRoleResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
