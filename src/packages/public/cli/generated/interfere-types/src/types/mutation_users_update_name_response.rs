pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationUsersUpdateNameResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationUsersUpdateNameResponse {
    pub fn builder() -> MutationUsersUpdateNameResponseBuilder {
        <MutationUsersUpdateNameResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationUsersUpdateNameResponseBuilder {
    success: Option<bool>,
}

impl MutationUsersUpdateNameResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationUsersUpdateNameResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationUsersUpdateNameResponseBuilder::success)
    pub fn build(self) -> Result<MutationUsersUpdateNameResponse, BuildError> {
        Ok(MutationUsersUpdateNameResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
