pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RemoveUsersRequestArgs {
    #[serde(rename = "userId")]
    #[serde(default)]
    pub user_id: String,
}

impl RemoveUsersRequestArgs {
    pub fn builder() -> RemoveUsersRequestArgsBuilder {
        <RemoveUsersRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RemoveUsersRequestArgsBuilder {
    user_id: Option<String>,
}

impl RemoveUsersRequestArgsBuilder {
    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RemoveUsersRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`user_id`](RemoveUsersRequestArgsBuilder::user_id)
    pub fn build(self) -> Result<RemoveUsersRequestArgs, BuildError> {
        Ok(RemoveUsersRequestArgs {
            user_id: self.user_id.ok_or_else(|| BuildError::missing_field("user_id"))?,
        })
    }
}
