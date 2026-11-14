pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SendSlackTestMessageRequest {
    pub category: SendSlackTestMessageRequestCategory,
    #[serde(rename = "idempotencyKey")]
    #[serde(default)]
    pub idempotency_key: String,
}

impl SendSlackTestMessageRequest {
    pub fn builder() -> SendSlackTestMessageRequestBuilder {
        <SendSlackTestMessageRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SendSlackTestMessageRequestBuilder {
    category: Option<SendSlackTestMessageRequestCategory>,
    idempotency_key: Option<String>,
}

impl SendSlackTestMessageRequestBuilder {
    pub fn category(mut self, value: SendSlackTestMessageRequestCategory) -> Self {
        self.category = Some(value);
        self
    }

    pub fn idempotency_key(mut self, value: impl Into<String>) -> Self {
        self.idempotency_key = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SendSlackTestMessageRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`category`](SendSlackTestMessageRequestBuilder::category)
    /// - [`idempotency_key`](SendSlackTestMessageRequestBuilder::idempotency_key)
    pub fn build(self) -> Result<SendSlackTestMessageRequest, BuildError> {
        Ok(SendSlackTestMessageRequest {
            category: self.category.ok_or_else(|| BuildError::missing_field("category"))?,
            idempotency_key: self.idempotency_key.ok_or_else(|| BuildError::missing_field("idempotency_key"))?,
        })
    }
}

