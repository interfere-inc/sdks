pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateLocaleUsersRequestArgs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
}

impl UpdateLocaleUsersRequestArgs {
    pub fn builder() -> UpdateLocaleUsersRequestArgsBuilder {
        <UpdateLocaleUsersRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateLocaleUsersRequestArgsBuilder {
    locale: Option<String>,
}

impl UpdateLocaleUsersRequestArgsBuilder {
    pub fn locale(mut self, value: impl Into<String>) -> Self {
        self.locale = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateLocaleUsersRequestArgs`].
    pub fn build(self) -> Result<UpdateLocaleUsersRequestArgs, BuildError> {
        Ok(UpdateLocaleUsersRequestArgs {
            locale: self.locale,
        })
    }
}
