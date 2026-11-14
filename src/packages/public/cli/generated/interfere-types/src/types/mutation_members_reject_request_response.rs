pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationMembersRejectRequestResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationMembersRejectRequestResponse {
    pub fn builder() -> MutationMembersRejectRequestResponseBuilder {
        <MutationMembersRejectRequestResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationMembersRejectRequestResponseBuilder {
    success: Option<bool>,
}

impl MutationMembersRejectRequestResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationMembersRejectRequestResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationMembersRejectRequestResponseBuilder::success)
    pub fn build(self) -> Result<MutationMembersRejectRequestResponse, BuildError> {
        Ok(MutationMembersRejectRequestResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
