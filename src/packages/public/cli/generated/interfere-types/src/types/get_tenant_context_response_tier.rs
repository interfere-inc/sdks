pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GetTenantContextResponseTier {
    #[serde(default)]
    pub value: String,
    pub shade: GetTenantContextResponseTierShade,
}

impl GetTenantContextResponseTier {
    pub fn builder() -> GetTenantContextResponseTierBuilder {
        <GetTenantContextResponseTierBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetTenantContextResponseTierBuilder {
    value: Option<String>,
    shade: Option<GetTenantContextResponseTierShade>,
}

impl GetTenantContextResponseTierBuilder {
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn shade(mut self, value: GetTenantContextResponseTierShade) -> Self {
        self.shade = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetTenantContextResponseTier`].
    /// This method will fail if any of the following fields are not set:
    /// - [`value`](GetTenantContextResponseTierBuilder::value)
    /// - [`shade`](GetTenantContextResponseTierBuilder::shade)
    pub fn build(self) -> Result<GetTenantContextResponseTier, BuildError> {
        Ok(GetTenantContextResponseTier {
            value: self.value.ok_or_else(|| BuildError::missing_field("value"))?,
            shade: self.shade.ok_or_else(|| BuildError::missing_field("shade"))?,
        })
    }
}
