pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateNameUsersRequestArgs {
    #[serde(rename = "firstName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
}

impl UpdateNameUsersRequestArgs {
    pub fn builder() -> UpdateNameUsersRequestArgsBuilder {
        <UpdateNameUsersRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateNameUsersRequestArgsBuilder {
    first_name: Option<String>,
    last_name: Option<String>,
}

impl UpdateNameUsersRequestArgsBuilder {
    pub fn first_name(mut self, value: impl Into<String>) -> Self {
        self.first_name = Some(value.into());
        self
    }

    pub fn last_name(mut self, value: impl Into<String>) -> Self {
        self.last_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateNameUsersRequestArgs`].
    pub fn build(self) -> Result<UpdateNameUsersRequestArgs, BuildError> {
        Ok(UpdateNameUsersRequestArgs {
            first_name: self.first_name,
            last_name: self.last_name,
        })
    }
}
