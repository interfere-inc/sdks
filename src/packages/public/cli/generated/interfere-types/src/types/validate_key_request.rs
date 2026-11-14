pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ValidateKeyRequest {
    /// The key exactly as found (environment variable, .env file, Worker secret or CI secret). It is checked live and never returned or logged.
    #[serde(default)]
    pub key: String,
    /// The kind the variable is meant to hold: public for INTERFERE_PUBLIC_KEY, secret for INTERFERE_API_KEY or INTERFERE_SECRET_KEY.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected: Option<ValidateKeyRequestExpected>,
    /// For a workspace secret key: the public key of the app the build uploads to, as the build plugins send it.
    #[serde(rename = "surfacePublicKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface_public_key: Option<String>,
}

impl ValidateKeyRequest {
    pub fn builder() -> ValidateKeyRequestBuilder {
        <ValidateKeyRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ValidateKeyRequestBuilder {
    key: Option<String>,
    expected: Option<ValidateKeyRequestExpected>,
    surface_public_key: Option<String>,
}

impl ValidateKeyRequestBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn expected(mut self, value: ValidateKeyRequestExpected) -> Self {
        self.expected = Some(value);
        self
    }

    pub fn surface_public_key(mut self, value: impl Into<String>) -> Self {
        self.surface_public_key = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ValidateKeyRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](ValidateKeyRequestBuilder::key)
    pub fn build(self) -> Result<ValidateKeyRequest, BuildError> {
        Ok(ValidateKeyRequest {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            expected: self.expected,
            surface_public_key: self.surface_public_key,
        })
    }
}

