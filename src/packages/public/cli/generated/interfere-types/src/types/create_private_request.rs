pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreatePrivateRequest {
    /// Surface URL slug, using lowercase letters, numbers, and hyphens.
    #[serde(rename = "surfaceSlug")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface_slug: Option<String>,
    #[serde(rename = "idempotencyKey")]
    #[serde(default)]
    pub idempotency_key: String,
    /// Display name of the key.
    #[serde(default)]
    pub name: String,
    /// Permission scopes granted to the key.
    #[serde(default)]
    pub scopes: Vec<CreatePrivateRequestScopesItem>,
    /// Positive lifetime in seconds, or null for no expiry.
    #[serde(rename = "secondsUntilExpiration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seconds_until_expiration: Option<i64>,
}

impl CreatePrivateRequest {
    pub fn builder() -> CreatePrivateRequestBuilder {
        <CreatePrivateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePrivateRequestBuilder {
    surface_slug: Option<String>,
    idempotency_key: Option<String>,
    name: Option<String>,
    scopes: Option<Vec<CreatePrivateRequestScopesItem>>,
    seconds_until_expiration: Option<i64>,
}

impl CreatePrivateRequestBuilder {
    pub fn surface_slug(mut self, value: impl Into<String>) -> Self {
        self.surface_slug = Some(value.into());
        self
    }

    pub fn idempotency_key(mut self, value: impl Into<String>) -> Self {
        self.idempotency_key = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn scopes(mut self, value: Vec<CreatePrivateRequestScopesItem>) -> Self {
        self.scopes = Some(value);
        self
    }

    pub fn seconds_until_expiration(mut self, value: i64) -> Self {
        self.seconds_until_expiration = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreatePrivateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`idempotency_key`](CreatePrivateRequestBuilder::idempotency_key)
    /// - [`name`](CreatePrivateRequestBuilder::name)
    /// - [`scopes`](CreatePrivateRequestBuilder::scopes)
    pub fn build(self) -> Result<CreatePrivateRequest, BuildError> {
        Ok(CreatePrivateRequest {
            surface_slug: self.surface_slug,
            idempotency_key: self.idempotency_key.ok_or_else(|| BuildError::missing_field("idempotency_key"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            scopes: self.scopes.ok_or_else(|| BuildError::missing_field("scopes"))?,
            seconds_until_expiration: self.seconds_until_expiration,
        })
    }
}

