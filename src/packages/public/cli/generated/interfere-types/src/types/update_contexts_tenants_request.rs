pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateContextsTenantsRequest {
    #[serde(default)]
    pub tenants: Vec<UpdateContextsTenantsRequestTenantsItem>,
}

impl UpdateContextsTenantsRequest {
    pub fn builder() -> UpdateContextsTenantsRequestBuilder {
        <UpdateContextsTenantsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateContextsTenantsRequestBuilder {
    tenants: Option<Vec<UpdateContextsTenantsRequestTenantsItem>>,
}

impl UpdateContextsTenantsRequestBuilder {
    pub fn tenants(mut self, value: Vec<UpdateContextsTenantsRequestTenantsItem>) -> Self {
        self.tenants = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateContextsTenantsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tenants`](UpdateContextsTenantsRequestBuilder::tenants)
    pub fn build(self) -> Result<UpdateContextsTenantsRequest, BuildError> {
        Ok(UpdateContextsTenantsRequest {
            tenants: self.tenants.ok_or_else(|| BuildError::missing_field("tenants"))?,
        })
    }
}

