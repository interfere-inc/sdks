pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetTenantsRequest {
    #[serde(default)]
    pub args: GetTenantsRequestArgs,
}

impl GetTenantsRequest {
    pub fn builder() -> GetTenantsRequestBuilder {
        <GetTenantsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetTenantsRequestBuilder {
    args: Option<GetTenantsRequestArgs>,
}

impl GetTenantsRequestBuilder {
    pub fn args(mut self, value: GetTenantsRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetTenantsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](GetTenantsRequestBuilder::args)
    pub fn build(self) -> Result<GetTenantsRequest, BuildError> {
        Ok(GetTenantsRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

