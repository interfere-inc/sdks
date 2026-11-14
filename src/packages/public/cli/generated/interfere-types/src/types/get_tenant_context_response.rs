pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetTenantContextResponse {
    #[serde(rename = "endUserOrgId")]
    #[serde(default)]
    pub end_user_org_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<GetTenantContextResponseTier>,
}

impl GetTenantContextResponse {
    pub fn builder() -> GetTenantContextResponseBuilder {
        <GetTenantContextResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetTenantContextResponseBuilder {
    end_user_org_id: Option<String>,
    tier: Option<GetTenantContextResponseTier>,
}

impl GetTenantContextResponseBuilder {
    pub fn end_user_org_id(mut self, value: impl Into<String>) -> Self {
        self.end_user_org_id = Some(value.into());
        self
    }

    pub fn tier(mut self, value: GetTenantContextResponseTier) -> Self {
        self.tier = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetTenantContextResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`end_user_org_id`](GetTenantContextResponseBuilder::end_user_org_id)
    pub fn build(self) -> Result<GetTenantContextResponse, BuildError> {
        Ok(GetTenantContextResponse {
            end_user_org_id: self.end_user_org_id.ok_or_else(|| BuildError::missing_field("end_user_org_id"))?,
            tier: self.tier,
        })
    }
}
