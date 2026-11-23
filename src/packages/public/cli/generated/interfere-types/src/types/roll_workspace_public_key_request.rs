pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RollWorkspacePublicKeyRequest {
    /// Milliseconds since the Unix epoch in UTC.
    #[serde(rename = "expectedUpdatedAt")]
    #[serde(default)]
    pub expected_updated_at: i64,
    /// Seconds the previous value keeps authenticating after rotation. Defaults to seven days because every app in the workspace ships the same key.
    #[serde(rename = "retireAfterSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retire_after_seconds: Option<String>,
}

impl RollWorkspacePublicKeyRequest {
    pub fn builder() -> RollWorkspacePublicKeyRequestBuilder {
        <RollWorkspacePublicKeyRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RollWorkspacePublicKeyRequestBuilder {
    expected_updated_at: Option<i64>,
    retire_after_seconds: Option<String>,
}

impl RollWorkspacePublicKeyRequestBuilder {
    pub fn expected_updated_at(mut self, value: i64) -> Self {
        self.expected_updated_at = Some(value);
        self
    }

    pub fn retire_after_seconds(mut self, value: impl Into<String>) -> Self {
        self.retire_after_seconds = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RollWorkspacePublicKeyRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`expected_updated_at`](RollWorkspacePublicKeyRequestBuilder::expected_updated_at)
    pub fn build(self) -> Result<RollWorkspacePublicKeyRequest, BuildError> {
        Ok(RollWorkspacePublicKeyRequest {
            expected_updated_at: self.expected_updated_at.ok_or_else(|| BuildError::missing_field("expected_updated_at"))?,
            retire_after_seconds: self.retire_after_seconds,
        })
    }
}

