pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateTimezoneUsersRequestArgs {
    #[serde(default)]
    pub timezone: String,
}

impl UpdateTimezoneUsersRequestArgs {
    pub fn builder() -> UpdateTimezoneUsersRequestArgsBuilder {
        <UpdateTimezoneUsersRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateTimezoneUsersRequestArgsBuilder {
    timezone: Option<String>,
}

impl UpdateTimezoneUsersRequestArgsBuilder {
    pub fn timezone(mut self, value: impl Into<String>) -> Self {
        self.timezone = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateTimezoneUsersRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`timezone`](UpdateTimezoneUsersRequestArgsBuilder::timezone)
    pub fn build(self) -> Result<UpdateTimezoneUsersRequestArgs, BuildError> {
        Ok(UpdateTimezoneUsersRequestArgs {
            timezone: self.timezone.ok_or_else(|| BuildError::missing_field("timezone"))?,
        })
    }
}
