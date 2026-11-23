pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct KnockTokenResponse {
    #[serde(rename = "expiresAt")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub expires_at: f64,
    #[serde(rename = "userToken")]
    #[serde(default)]
    pub user_token: String,
}

impl KnockTokenResponse {
    pub fn builder() -> KnockTokenResponseBuilder {
        <KnockTokenResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct KnockTokenResponseBuilder {
    expires_at: Option<f64>,
    user_token: Option<String>,
}

impl KnockTokenResponseBuilder {
    pub fn expires_at(mut self, value: f64) -> Self {
        self.expires_at = Some(value);
        self
    }

    pub fn user_token(mut self, value: impl Into<String>) -> Self {
        self.user_token = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`KnockTokenResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`expires_at`](KnockTokenResponseBuilder::expires_at)
    /// - [`user_token`](KnockTokenResponseBuilder::user_token)
    pub fn build(self) -> Result<KnockTokenResponse, BuildError> {
        Ok(KnockTokenResponse {
            expires_at: self.expires_at.ok_or_else(|| BuildError::missing_field("expires_at"))?,
            user_token: self.user_token.ok_or_else(|| BuildError::missing_field("user_token"))?,
        })
    }
}
