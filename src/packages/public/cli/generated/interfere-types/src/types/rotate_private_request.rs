pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RotatePrivateRequest {
    #[serde(rename = "expectedUpdatedAt")]
    #[serde(default)]
    pub expected_updated_at: i64,
    #[serde(rename = "retireAfterSeconds")]
    #[serde(default)]
    pub retire_after_seconds: String,
    #[serde(rename = "idempotencyKey")]
    #[serde(default)]
    pub idempotency_key: String,
}

impl RotatePrivateRequest {
    pub fn builder() -> RotatePrivateRequestBuilder {
        <RotatePrivateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RotatePrivateRequestBuilder {
    expected_updated_at: Option<i64>,
    retire_after_seconds: Option<String>,
    idempotency_key: Option<String>,
}

impl RotatePrivateRequestBuilder {
    pub fn expected_updated_at(mut self, value: i64) -> Self {
        self.expected_updated_at = Some(value);
        self
    }

    pub fn retire_after_seconds(mut self, value: impl Into<String>) -> Self {
        self.retire_after_seconds = Some(value.into());
        self
    }

    pub fn idempotency_key(mut self, value: impl Into<String>) -> Self {
        self.idempotency_key = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RotatePrivateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`expected_updated_at`](RotatePrivateRequestBuilder::expected_updated_at)
    /// - [`retire_after_seconds`](RotatePrivateRequestBuilder::retire_after_seconds)
    /// - [`idempotency_key`](RotatePrivateRequestBuilder::idempotency_key)
    pub fn build(self) -> Result<RotatePrivateRequest, BuildError> {
        Ok(RotatePrivateRequest {
            expected_updated_at: self.expected_updated_at.ok_or_else(|| BuildError::missing_field("expected_updated_at"))?,
            retire_after_seconds: self.retire_after_seconds.ok_or_else(|| BuildError::missing_field("retire_after_seconds"))?,
            idempotency_key: self.idempotency_key.ok_or_else(|| BuildError::missing_field("idempotency_key"))?,
        })
    }
}

