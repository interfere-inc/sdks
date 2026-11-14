pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationUsersUpdateTimezoneResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationUsersUpdateTimezoneResponse {
    pub fn builder() -> MutationUsersUpdateTimezoneResponseBuilder {
        <MutationUsersUpdateTimezoneResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationUsersUpdateTimezoneResponseBuilder {
    success: Option<bool>,
}

impl MutationUsersUpdateTimezoneResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationUsersUpdateTimezoneResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationUsersUpdateTimezoneResponseBuilder::success)
    pub fn build(self) -> Result<MutationUsersUpdateTimezoneResponse, BuildError> {
        Ok(MutationUsersUpdateTimezoneResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
