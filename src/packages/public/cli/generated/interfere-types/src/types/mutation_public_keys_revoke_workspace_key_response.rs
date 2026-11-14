pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationPublicKeysRevokeWorkspaceKeyResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationPublicKeysRevokeWorkspaceKeyResponse {
    pub fn builder() -> MutationPublicKeysRevokeWorkspaceKeyResponseBuilder {
        <MutationPublicKeysRevokeWorkspaceKeyResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationPublicKeysRevokeWorkspaceKeyResponseBuilder {
    success: Option<bool>,
}

impl MutationPublicKeysRevokeWorkspaceKeyResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationPublicKeysRevokeWorkspaceKeyResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationPublicKeysRevokeWorkspaceKeyResponseBuilder::success)
    pub fn build(self) -> Result<MutationPublicKeysRevokeWorkspaceKeyResponse, BuildError> {
        Ok(MutationPublicKeysRevokeWorkspaceKeyResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
