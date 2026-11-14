pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationMembersRemoveResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationMembersRemoveResponse {
    pub fn builder() -> MutationMembersRemoveResponseBuilder {
        <MutationMembersRemoveResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationMembersRemoveResponseBuilder {
    success: Option<bool>,
}

impl MutationMembersRemoveResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationMembersRemoveResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationMembersRemoveResponseBuilder::success)
    pub fn build(self) -> Result<MutationMembersRemoveResponse, BuildError> {
        Ok(MutationMembersRemoveResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
