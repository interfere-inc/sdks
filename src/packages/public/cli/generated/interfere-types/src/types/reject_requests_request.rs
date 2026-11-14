pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RejectRequestsRequest {
    #[serde(default)]
    pub args: RejectRequestsRequestArgs,
}

impl RejectRequestsRequest {
    pub fn builder() -> RejectRequestsRequestBuilder {
        <RejectRequestsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RejectRequestsRequestBuilder {
    args: Option<RejectRequestsRequestArgs>,
}

impl RejectRequestsRequestBuilder {
    pub fn args(mut self, value: RejectRequestsRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RejectRequestsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](RejectRequestsRequestBuilder::args)
    pub fn build(self) -> Result<RejectRequestsRequest, BuildError> {
        Ok(RejectRequestsRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

