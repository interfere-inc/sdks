pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateSurfaceRequestApiKey {
    /// Permission scopes granted to the key.
    #[serde(default)]
    pub scopes: Vec<CreateSurfaceRequestApiKeyScopesItem>,
    /// Positive lifetime in seconds, or null for no expiry.
    #[serde(rename = "secondsUntilExpiration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seconds_until_expiration: Option<i64>,
}

impl CreateSurfaceRequestApiKey {
    pub fn builder() -> CreateSurfaceRequestApiKeyBuilder {
        <CreateSurfaceRequestApiKeyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateSurfaceRequestApiKeyBuilder {
    scopes: Option<Vec<CreateSurfaceRequestApiKeyScopesItem>>,
    seconds_until_expiration: Option<i64>,
}

impl CreateSurfaceRequestApiKeyBuilder {
    pub fn scopes(mut self, value: Vec<CreateSurfaceRequestApiKeyScopesItem>) -> Self {
        self.scopes = Some(value);
        self
    }

    pub fn seconds_until_expiration(mut self, value: i64) -> Self {
        self.seconds_until_expiration = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateSurfaceRequestApiKey`].
    /// This method will fail if any of the following fields are not set:
    /// - [`scopes`](CreateSurfaceRequestApiKeyBuilder::scopes)
    pub fn build(self) -> Result<CreateSurfaceRequestApiKey, BuildError> {
        Ok(CreateSurfaceRequestApiKey {
            scopes: self.scopes.ok_or_else(|| BuildError::missing_field("scopes"))?,
            seconds_until_expiration: self.seconds_until_expiration,
        })
    }
}
