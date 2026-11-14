pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationUsersUpdateLocaleResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationUsersUpdateLocaleResponse {
    pub fn builder() -> MutationUsersUpdateLocaleResponseBuilder {
        <MutationUsersUpdateLocaleResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationUsersUpdateLocaleResponseBuilder {
    success: Option<bool>,
}

impl MutationUsersUpdateLocaleResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationUsersUpdateLocaleResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationUsersUpdateLocaleResponseBuilder::success)
    pub fn build(self) -> Result<MutationUsersUpdateLocaleResponse, BuildError> {
        Ok(MutationUsersUpdateLocaleResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
