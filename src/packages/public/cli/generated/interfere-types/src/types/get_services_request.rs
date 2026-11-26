pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetServicesRequest {
    #[serde(default)]
    pub args: GetServicesRequestArgs,
}

impl GetServicesRequest {
    pub fn builder() -> GetServicesRequestBuilder {
        <GetServicesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetServicesRequestBuilder {
    args: Option<GetServicesRequestArgs>,
}

impl GetServicesRequestBuilder {
    pub fn args(mut self, value: GetServicesRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetServicesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](GetServicesRequestBuilder::args)
    pub fn build(self) -> Result<GetServicesRequest, BuildError> {
        Ok(GetServicesRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

