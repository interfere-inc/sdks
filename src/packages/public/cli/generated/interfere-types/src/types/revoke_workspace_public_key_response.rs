pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RevokeWorkspacePublicKeyResponse {
    #[serde(default)]
    pub success: bool,
}

impl RevokeWorkspacePublicKeyResponse {
    pub fn builder() -> RevokeWorkspacePublicKeyResponseBuilder {
        <RevokeWorkspacePublicKeyResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RevokeWorkspacePublicKeyResponseBuilder {
    success: Option<bool>,
}

impl RevokeWorkspacePublicKeyResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RevokeWorkspacePublicKeyResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](RevokeWorkspacePublicKeyResponseBuilder::success)
    pub fn build(self) -> Result<RevokeWorkspacePublicKeyResponse, BuildError> {
        Ok(RevokeWorkspacePublicKeyResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
