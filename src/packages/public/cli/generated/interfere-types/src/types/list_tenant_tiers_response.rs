pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListTenantTiersResponse {
    #[serde(default)]
    pub tiers: Vec<CreateTenantTierResponse>,
}

impl ListTenantTiersResponse {
    pub fn builder() -> ListTenantTiersResponseBuilder {
        <ListTenantTiersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListTenantTiersResponseBuilder {
    tiers: Option<Vec<CreateTenantTierResponse>>,
}

impl ListTenantTiersResponseBuilder {
    pub fn tiers(mut self, value: Vec<CreateTenantTierResponse>) -> Self {
        self.tiers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListTenantTiersResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tiers`](ListTenantTiersResponseBuilder::tiers)
    pub fn build(self) -> Result<ListTenantTiersResponse, BuildError> {
        Ok(ListTenantTiersResponse {
            tiers: self.tiers.ok_or_else(|| BuildError::missing_field("tiers"))?,
        })
    }
}
