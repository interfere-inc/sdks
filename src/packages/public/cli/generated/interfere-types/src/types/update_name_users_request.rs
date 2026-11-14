pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateNameUsersRequest {
    #[serde(default)]
    pub args: UpdateNameUsersRequestArgs,
}

impl UpdateNameUsersRequest {
    pub fn builder() -> UpdateNameUsersRequestBuilder {
        <UpdateNameUsersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateNameUsersRequestBuilder {
    args: Option<UpdateNameUsersRequestArgs>,
}

impl UpdateNameUsersRequestBuilder {
    pub fn args(mut self, value: UpdateNameUsersRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateNameUsersRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](UpdateNameUsersRequestBuilder::args)
    pub fn build(self) -> Result<UpdateNameUsersRequest, BuildError> {
        Ok(UpdateNameUsersRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

