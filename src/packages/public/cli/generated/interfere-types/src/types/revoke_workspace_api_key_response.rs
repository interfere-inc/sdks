pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RevokeWorkspaceApiKeyResponse {
    #[serde(default)]
    pub success: bool,
}

impl RevokeWorkspaceApiKeyResponse {
    pub fn builder() -> RevokeWorkspaceApiKeyResponseBuilder {
        <RevokeWorkspaceApiKeyResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RevokeWorkspaceApiKeyResponseBuilder {
    success: Option<bool>,
}

impl RevokeWorkspaceApiKeyResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RevokeWorkspaceApiKeyResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](RevokeWorkspaceApiKeyResponseBuilder::success)
    pub fn build(self) -> Result<RevokeWorkspaceApiKeyResponse, BuildError> {
        Ok(RevokeWorkspaceApiKeyResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
