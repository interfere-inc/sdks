pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateLocaleUsersRequest {
    #[serde(default)]
    pub args: UpdateLocaleUsersRequestArgs,
}

impl UpdateLocaleUsersRequest {
    pub fn builder() -> UpdateLocaleUsersRequestBuilder {
        <UpdateLocaleUsersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateLocaleUsersRequestBuilder {
    args: Option<UpdateLocaleUsersRequestArgs>,
}

impl UpdateLocaleUsersRequestBuilder {
    pub fn args(mut self, value: UpdateLocaleUsersRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateLocaleUsersRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](UpdateLocaleUsersRequestBuilder::args)
    pub fn build(self) -> Result<UpdateLocaleUsersRequest, BuildError> {
        Ok(UpdateLocaleUsersRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

