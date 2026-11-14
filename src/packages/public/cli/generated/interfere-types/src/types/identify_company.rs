pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Company attributes attached when identifying a session. The tier is a workspace-managed label (1 to 100 characters, trimmed); unknown tiers are created on first use.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct IdentifyCompany {
    #[serde(default)]
    pub tier: String,
}

impl IdentifyCompany {
    pub fn builder() -> IdentifyCompanyBuilder {
        <IdentifyCompanyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IdentifyCompanyBuilder {
    tier: Option<String>,
}

impl IdentifyCompanyBuilder {
    pub fn tier(mut self, value: impl Into<String>) -> Self {
        self.tier = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`IdentifyCompany`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tier`](IdentifyCompanyBuilder::tier)
    pub fn build(self) -> Result<IdentifyCompany, BuildError> {
        Ok(IdentifyCompany {
            tier: self.tier.ok_or_else(|| BuildError::missing_field("tier"))?,
        })
    }
}
