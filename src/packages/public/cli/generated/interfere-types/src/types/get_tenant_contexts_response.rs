pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetTenantContextsResponse {
    #[serde(default)]
    pub tenants: Vec<GetTenantContextResponse>,
}

impl GetTenantContextsResponse {
    pub fn builder() -> GetTenantContextsResponseBuilder {
        <GetTenantContextsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetTenantContextsResponseBuilder {
    tenants: Option<Vec<GetTenantContextResponse>>,
}

impl GetTenantContextsResponseBuilder {
    pub fn tenants(mut self, value: Vec<GetTenantContextResponse>) -> Self {
        self.tenants = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetTenantContextsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tenants`](GetTenantContextsResponseBuilder::tenants)
    pub fn build(self) -> Result<GetTenantContextsResponse, BuildError> {
        Ok(GetTenantContextsResponse {
            tenants: self.tenants.ok_or_else(|| BuildError::missing_field("tenants"))?,
        })
    }
}
