pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ValidateKeyResponse {
    pub outcome: ValidateKeyResponseOutcome,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<ValidateKeyResponseKind>,
    #[serde(rename = "keyRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_region: Option<ValidateKeyResponseKeyRegion>,
    #[serde(rename = "surfaceSlug")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface_slug: Option<String>,
    #[serde(rename = "nextStep")]
    #[serde(default)]
    pub next_step: String,
}

impl ValidateKeyResponse {
    pub fn builder() -> ValidateKeyResponseBuilder {
        <ValidateKeyResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ValidateKeyResponseBuilder {
    outcome: Option<ValidateKeyResponseOutcome>,
    kind: Option<ValidateKeyResponseKind>,
    key_region: Option<ValidateKeyResponseKeyRegion>,
    surface_slug: Option<String>,
    next_step: Option<String>,
}

impl ValidateKeyResponseBuilder {
    pub fn outcome(mut self, value: ValidateKeyResponseOutcome) -> Self {
        self.outcome = Some(value);
        self
    }

    pub fn kind(mut self, value: ValidateKeyResponseKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn key_region(mut self, value: ValidateKeyResponseKeyRegion) -> Self {
        self.key_region = Some(value);
        self
    }

    pub fn surface_slug(mut self, value: impl Into<String>) -> Self {
        self.surface_slug = Some(value.into());
        self
    }

    pub fn next_step(mut self, value: impl Into<String>) -> Self {
        self.next_step = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ValidateKeyResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`outcome`](ValidateKeyResponseBuilder::outcome)
    /// - [`next_step`](ValidateKeyResponseBuilder::next_step)
    pub fn build(self) -> Result<ValidateKeyResponse, BuildError> {
        Ok(ValidateKeyResponse {
            outcome: self.outcome.ok_or_else(|| BuildError::missing_field("outcome"))?,
            kind: self.kind,
            key_region: self.key_region,
            surface_slug: self.surface_slug,
            next_step: self.next_step.ok_or_else(|| BuildError::missing_field("next_step"))?,
        })
    }
}
