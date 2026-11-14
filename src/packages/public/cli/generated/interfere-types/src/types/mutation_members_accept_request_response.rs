pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationMembersAcceptRequestResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationMembersAcceptRequestResponse {
    pub fn builder() -> MutationMembersAcceptRequestResponseBuilder {
        <MutationMembersAcceptRequestResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationMembersAcceptRequestResponseBuilder {
    success: Option<bool>,
}

impl MutationMembersAcceptRequestResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationMembersAcceptRequestResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationMembersAcceptRequestResponseBuilder::success)
    pub fn build(self) -> Result<MutationMembersAcceptRequestResponse, BuildError> {
        Ok(MutationMembersAcceptRequestResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
