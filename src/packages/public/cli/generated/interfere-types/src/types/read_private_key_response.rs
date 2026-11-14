pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReadPrivateKeyResponse {
    /// Stable private key identifier.
    #[serde(default)]
    pub id: String,
    /// Identifier of the currently active key version.
    #[serde(default)]
    pub version: String,
    /// Display name of the key.
    #[serde(default)]
    pub name: String,
    /// Surface URL slug, using lowercase letters, numbers, and hyphens.
    #[serde(rename = "surfaceSlug")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface_slug: Option<String>,
    /// Permission scopes granted to the key.
    #[serde(default)]
    pub scopes: Vec<ReadPrivateKeyResponseScopesItem>,
    /// Positive lifetime in seconds, or null for no expiry.
    #[serde(rename = "secondsUntilExpiration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seconds_until_expiration: Option<i64>,
    /// Expiration as Unix milliseconds, or null for no expiry.
    #[serde(rename = "expiresAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
    #[serde(default)]
    pub revoked: bool,
}

impl ReadPrivateKeyResponse {
    pub fn builder() -> ReadPrivateKeyResponseBuilder {
        <ReadPrivateKeyResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReadPrivateKeyResponseBuilder {
    id: Option<String>,
    version: Option<String>,
    name: Option<String>,
    surface_slug: Option<String>,
    scopes: Option<Vec<ReadPrivateKeyResponseScopesItem>>,
    seconds_until_expiration: Option<i64>,
    expires_at: Option<i64>,
    revoked: Option<bool>,
}

impl ReadPrivateKeyResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn version(mut self, value: impl Into<String>) -> Self {
        self.version = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn surface_slug(mut self, value: impl Into<String>) -> Self {
        self.surface_slug = Some(value.into());
        self
    }

    pub fn scopes(mut self, value: Vec<ReadPrivateKeyResponseScopesItem>) -> Self {
        self.scopes = Some(value);
        self
    }

    pub fn seconds_until_expiration(mut self, value: i64) -> Self {
        self.seconds_until_expiration = Some(value);
        self
    }

    pub fn expires_at(mut self, value: i64) -> Self {
        self.expires_at = Some(value);
        self
    }

    pub fn revoked(mut self, value: bool) -> Self {
        self.revoked = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReadPrivateKeyResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ReadPrivateKeyResponseBuilder::id)
    /// - [`version`](ReadPrivateKeyResponseBuilder::version)
    /// - [`name`](ReadPrivateKeyResponseBuilder::name)
    /// - [`scopes`](ReadPrivateKeyResponseBuilder::scopes)
    /// - [`revoked`](ReadPrivateKeyResponseBuilder::revoked)
    pub fn build(self) -> Result<ReadPrivateKeyResponse, BuildError> {
        Ok(ReadPrivateKeyResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            version: self.version.ok_or_else(|| BuildError::missing_field("version"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            surface_slug: self.surface_slug,
            scopes: self.scopes.ok_or_else(|| BuildError::missing_field("scopes"))?,
            seconds_until_expiration: self.seconds_until_expiration,
            expires_at: self.expires_at,
            revoked: self.revoked.ok_or_else(|| BuildError::missing_field("revoked"))?,
        })
    }
}
