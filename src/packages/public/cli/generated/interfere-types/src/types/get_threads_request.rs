pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetThreadsRequest {
    #[serde(default)]
    pub args: GetThreadsRequestArgs,
}

impl GetThreadsRequest {
    pub fn builder() -> GetThreadsRequestBuilder {
        <GetThreadsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetThreadsRequestBuilder {
    args: Option<GetThreadsRequestArgs>,
}

impl GetThreadsRequestBuilder {
    pub fn args(mut self, value: GetThreadsRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetThreadsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](GetThreadsRequestBuilder::args)
    pub fn build(self) -> Result<GetThreadsRequest, BuildError> {
        Ok(GetThreadsRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

