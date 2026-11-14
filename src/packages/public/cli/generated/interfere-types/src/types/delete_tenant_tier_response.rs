pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteTenantTierResponse {
    #[serde(rename = "clearedCompanyCount")]
    #[serde(default)]
    pub cleared_company_count: i64,
}

impl DeleteTenantTierResponse {
    pub fn builder() -> DeleteTenantTierResponseBuilder {
        <DeleteTenantTierResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteTenantTierResponseBuilder {
    cleared_company_count: Option<i64>,
}

impl DeleteTenantTierResponseBuilder {
    pub fn cleared_company_count(mut self, value: i64) -> Self {
        self.cleared_company_count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeleteTenantTierResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`cleared_company_count`](DeleteTenantTierResponseBuilder::cleared_company_count)
    pub fn build(self) -> Result<DeleteTenantTierResponse, BuildError> {
        Ok(DeleteTenantTierResponse {
            cleared_company_count: self.cleared_company_count.ok_or_else(|| BuildError::missing_field("cleared_company_count"))?,
        })
    }
}
