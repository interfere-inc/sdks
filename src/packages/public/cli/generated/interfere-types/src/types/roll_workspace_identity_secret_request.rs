pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RollWorkspaceIdentitySecretRequest {
    /// Seconds the previous secret keeps verifying signatures after rotation, so servers can deploy the new value. Defaults to one day; ignored when the workspace has no secret yet.
    #[serde(rename = "retireAfterSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retire_after_seconds: Option<String>,
}

impl RollWorkspaceIdentitySecretRequest {
    pub fn builder() -> RollWorkspaceIdentitySecretRequestBuilder {
        <RollWorkspaceIdentitySecretRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RollWorkspaceIdentitySecretRequestBuilder {
    retire_after_seconds: Option<String>,
}

impl RollWorkspaceIdentitySecretRequestBuilder {
    pub fn retire_after_seconds(mut self, value: impl Into<String>) -> Self {
        self.retire_after_seconds = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RollWorkspaceIdentitySecretRequest`].
    pub fn build(self) -> Result<RollWorkspaceIdentitySecretRequest, BuildError> {
        Ok(RollWorkspaceIdentitySecretRequest {
            retire_after_seconds: self.retire_after_seconds,
        })
    }
}

