pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RemoveUsersRequest {
    #[serde(default)]
    pub args: RemoveUsersRequestArgs,
}

impl RemoveUsersRequest {
    pub fn builder() -> RemoveUsersRequestBuilder {
        <RemoveUsersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RemoveUsersRequestBuilder {
    args: Option<RemoveUsersRequestArgs>,
}

impl RemoveUsersRequestBuilder {
    pub fn args(mut self, value: RemoveUsersRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RemoveUsersRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](RemoveUsersRequestBuilder::args)
    pub fn build(self) -> Result<RemoveUsersRequest, BuildError> {
        Ok(RemoveUsersRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

