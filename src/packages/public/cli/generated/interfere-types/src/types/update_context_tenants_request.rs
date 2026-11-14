pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateContextTenantsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}

impl UpdateContextTenantsRequest {
    pub fn builder() -> UpdateContextTenantsRequestBuilder {
        <UpdateContextTenantsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateContextTenantsRequestBuilder {
    tier: Option<String>,
}

impl UpdateContextTenantsRequestBuilder {
    pub fn tier(mut self, value: impl Into<String>) -> Self {
        self.tier = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateContextTenantsRequest`].
    pub fn build(self) -> Result<UpdateContextTenantsRequest, BuildError> {
        Ok(UpdateContextTenantsRequest {
            tier: self.tier,
        })
    }
}

