pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateTimezoneUsersRequest {
    #[serde(default)]
    pub args: UpdateTimezoneUsersRequestArgs,
}

impl UpdateTimezoneUsersRequest {
    pub fn builder() -> UpdateTimezoneUsersRequestBuilder {
        <UpdateTimezoneUsersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateTimezoneUsersRequestBuilder {
    args: Option<UpdateTimezoneUsersRequestArgs>,
}

impl UpdateTimezoneUsersRequestBuilder {
    pub fn args(mut self, value: UpdateTimezoneUsersRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateTimezoneUsersRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](UpdateTimezoneUsersRequestBuilder::args)
    pub fn build(self) -> Result<UpdateTimezoneUsersRequest, BuildError> {
        Ok(UpdateTimezoneUsersRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

