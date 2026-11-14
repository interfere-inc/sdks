pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UpdateRoleUsersRequestArgs {
    pub role: UpdateRoleUsersRequestArgsRole,
    #[serde(rename = "userId")]
    #[serde(default)]
    pub user_id: String,
}

impl UpdateRoleUsersRequestArgs {
    pub fn builder() -> UpdateRoleUsersRequestArgsBuilder {
        <UpdateRoleUsersRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateRoleUsersRequestArgsBuilder {
    role: Option<UpdateRoleUsersRequestArgsRole>,
    user_id: Option<String>,
}

impl UpdateRoleUsersRequestArgsBuilder {
    pub fn role(mut self, value: UpdateRoleUsersRequestArgsRole) -> Self {
        self.role = Some(value);
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateRoleUsersRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`role`](UpdateRoleUsersRequestArgsBuilder::role)
    /// - [`user_id`](UpdateRoleUsersRequestArgsBuilder::user_id)
    pub fn build(self) -> Result<UpdateRoleUsersRequestArgs, BuildError> {
        Ok(UpdateRoleUsersRequestArgs {
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
            user_id: self.user_id.ok_or_else(|| BuildError::missing_field("user_id"))?,
        })
    }
}
