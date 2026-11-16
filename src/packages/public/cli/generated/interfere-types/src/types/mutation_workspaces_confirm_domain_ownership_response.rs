pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationWorkspacesConfirmDomainOwnershipResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationWorkspacesConfirmDomainOwnershipResponse {
    pub fn builder() -> MutationWorkspacesConfirmDomainOwnershipResponseBuilder {
        <MutationWorkspacesConfirmDomainOwnershipResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationWorkspacesConfirmDomainOwnershipResponseBuilder {
    success: Option<bool>,
}

impl MutationWorkspacesConfirmDomainOwnershipResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationWorkspacesConfirmDomainOwnershipResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationWorkspacesConfirmDomainOwnershipResponseBuilder::success)
    pub fn build(self) -> Result<MutationWorkspacesConfirmDomainOwnershipResponse, BuildError> {
        Ok(MutationWorkspacesConfirmDomainOwnershipResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
