pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateTenantTierResponse {
    #[serde(default)]
    pub value: String,
    pub shade: CreateTenantTierResponseShade,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl CreateTenantTierResponse {
    pub fn builder() -> CreateTenantTierResponseBuilder {
        <CreateTenantTierResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateTenantTierResponseBuilder {
    value: Option<String>,
    shade: Option<CreateTenantTierResponseShade>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl CreateTenantTierResponseBuilder {
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn shade(mut self, value: CreateTenantTierResponseShade) -> Self {
        self.shade = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateTenantTierResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`value`](CreateTenantTierResponseBuilder::value)
    /// - [`shade`](CreateTenantTierResponseBuilder::shade)
    /// - [`created_at`](CreateTenantTierResponseBuilder::created_at)
    /// - [`updated_at`](CreateTenantTierResponseBuilder::updated_at)
    pub fn build(self) -> Result<CreateTenantTierResponse, BuildError> {
        Ok(CreateTenantTierResponse {
            value: self.value.ok_or_else(|| BuildError::missing_field("value"))?,
            shade: self.shade.ok_or_else(|| BuildError::missing_field("shade"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
