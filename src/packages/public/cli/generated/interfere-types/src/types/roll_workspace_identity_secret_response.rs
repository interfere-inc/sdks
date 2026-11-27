pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RollWorkspaceIdentitySecretResponse {
    /// Identify signing secret. Returned only by this call: store it on your server and never ship it to a browser.
    #[serde(default)]
    pub secret: String,
}

impl RollWorkspaceIdentitySecretResponse {
    pub fn builder() -> RollWorkspaceIdentitySecretResponseBuilder {
        <RollWorkspaceIdentitySecretResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RollWorkspaceIdentitySecretResponseBuilder {
    secret: Option<String>,
}

impl RollWorkspaceIdentitySecretResponseBuilder {
    pub fn secret(mut self, value: impl Into<String>) -> Self {
        self.secret = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RollWorkspaceIdentitySecretResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`secret`](RollWorkspaceIdentitySecretResponseBuilder::secret)
    pub fn build(self) -> Result<RollWorkspaceIdentitySecretResponse, BuildError> {
        Ok(RollWorkspaceIdentitySecretResponse {
            secret: self.secret.ok_or_else(|| BuildError::missing_field("secret"))?,
        })
    }
}
