pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AcceptRequestsRequest {
    pub args: AcceptRequestsRequestArgs,
}

impl AcceptRequestsRequest {
    pub fn builder() -> AcceptRequestsRequestBuilder {
        <AcceptRequestsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AcceptRequestsRequestBuilder {
    args: Option<AcceptRequestsRequestArgs>,
}

impl AcceptRequestsRequestBuilder {
    pub fn args(mut self, value: AcceptRequestsRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AcceptRequestsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](AcceptRequestsRequestBuilder::args)
    pub fn build(self) -> Result<AcceptRequestsRequest, BuildError> {
        Ok(AcceptRequestsRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

