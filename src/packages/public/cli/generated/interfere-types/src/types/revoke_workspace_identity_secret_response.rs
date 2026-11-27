pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RevokeWorkspaceIdentitySecretResponse {
    #[serde(default)]
    pub success: bool,
}

impl RevokeWorkspaceIdentitySecretResponse {
    pub fn builder() -> RevokeWorkspaceIdentitySecretResponseBuilder {
        <RevokeWorkspaceIdentitySecretResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RevokeWorkspaceIdentitySecretResponseBuilder {
    success: Option<bool>,
}

impl RevokeWorkspaceIdentitySecretResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RevokeWorkspaceIdentitySecretResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](RevokeWorkspaceIdentitySecretResponseBuilder::success)
    pub fn build(self) -> Result<RevokeWorkspaceIdentitySecretResponse, BuildError> {
        Ok(RevokeWorkspaceIdentitySecretResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
