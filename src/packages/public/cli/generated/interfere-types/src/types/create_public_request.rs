pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreatePublicRequest {
    #[serde(rename = "idempotencyKey")]
    #[serde(default)]
    pub idempotency_key: String,
    /// Display name of the key.
    #[serde(default)]
    pub name: String,
}

impl CreatePublicRequest {
    pub fn builder() -> CreatePublicRequestBuilder {
        <CreatePublicRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePublicRequestBuilder {
    idempotency_key: Option<String>,
    name: Option<String>,
}

impl CreatePublicRequestBuilder {
    pub fn idempotency_key(mut self, value: impl Into<String>) -> Self {
        self.idempotency_key = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreatePublicRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`idempotency_key`](CreatePublicRequestBuilder::idempotency_key)
    /// - [`name`](CreatePublicRequestBuilder::name)
    pub fn build(self) -> Result<CreatePublicRequest, BuildError> {
        Ok(CreatePublicRequest {
            idempotency_key: self.idempotency_key.ok_or_else(|| BuildError::missing_field("idempotency_key"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}

