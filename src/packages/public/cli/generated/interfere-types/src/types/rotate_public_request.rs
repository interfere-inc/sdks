pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RotatePublicRequest {
    #[serde(rename = "expectedUpdatedAt")]
    #[serde(default)]
    pub expected_updated_at: i64,
    #[serde(rename = "retireAfterSeconds")]
    #[serde(default)]
    pub retire_after_seconds: String,
}

impl RotatePublicRequest {
    pub fn builder() -> RotatePublicRequestBuilder {
        <RotatePublicRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RotatePublicRequestBuilder {
    expected_updated_at: Option<i64>,
    retire_after_seconds: Option<String>,
}

impl RotatePublicRequestBuilder {
    pub fn expected_updated_at(mut self, value: i64) -> Self {
        self.expected_updated_at = Some(value);
        self
    }

    pub fn retire_after_seconds(mut self, value: impl Into<String>) -> Self {
        self.retire_after_seconds = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RotatePublicRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`expected_updated_at`](RotatePublicRequestBuilder::expected_updated_at)
    /// - [`retire_after_seconds`](RotatePublicRequestBuilder::retire_after_seconds)
    pub fn build(self) -> Result<RotatePublicRequest, BuildError> {
        Ok(RotatePublicRequest {
            expected_updated_at: self.expected_updated_at.ok_or_else(|| BuildError::missing_field("expected_updated_at"))?,
            retire_after_seconds: self.retire_after_seconds.ok_or_else(|| BuildError::missing_field("retire_after_seconds"))?,
        })
    }
}

