pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UpdateRoleUsersRequest {
    pub args: UpdateRoleUsersRequestArgs,
}

impl UpdateRoleUsersRequest {
    pub fn builder() -> UpdateRoleUsersRequestBuilder {
        <UpdateRoleUsersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateRoleUsersRequestBuilder {
    args: Option<UpdateRoleUsersRequestArgs>,
}

impl UpdateRoleUsersRequestBuilder {
    pub fn args(mut self, value: UpdateRoleUsersRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateRoleUsersRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](UpdateRoleUsersRequestBuilder::args)
    pub fn build(self) -> Result<UpdateRoleUsersRequest, BuildError> {
        Ok(UpdateRoleUsersRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

