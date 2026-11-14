pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetByKeySessionsRequest {
    #[serde(default)]
    pub args: GetByKeySessionsRequestArgs,
}

impl GetByKeySessionsRequest {
    pub fn builder() -> GetByKeySessionsRequestBuilder {
        <GetByKeySessionsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetByKeySessionsRequestBuilder {
    args: Option<GetByKeySessionsRequestArgs>,
}

impl GetByKeySessionsRequestBuilder {
    pub fn args(mut self, value: GetByKeySessionsRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetByKeySessionsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](GetByKeySessionsRequestBuilder::args)
    pub fn build(self) -> Result<GetByKeySessionsRequest, BuildError> {
        Ok(GetByKeySessionsRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

