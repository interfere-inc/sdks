pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SeenUserTenantsRequest {
    pub args: SeenUserTenantsRequestArgs,
}

impl SeenUserTenantsRequest {
    pub fn builder() -> SeenUserTenantsRequestBuilder {
        <SeenUserTenantsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SeenUserTenantsRequestBuilder {
    args: Option<SeenUserTenantsRequestArgs>,
}

impl SeenUserTenantsRequestBuilder {
    pub fn args(mut self, value: SeenUserTenantsRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SeenUserTenantsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](SeenUserTenantsRequestBuilder::args)
    pub fn build(self) -> Result<SeenUserTenantsRequest, BuildError> {
        Ok(SeenUserTenantsRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

