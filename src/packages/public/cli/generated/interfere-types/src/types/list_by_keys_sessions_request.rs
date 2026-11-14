pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListByKeysSessionsRequest {
    #[serde(default)]
    pub args: ListByKeysSessionsRequestArgs,
}

impl ListByKeysSessionsRequest {
    pub fn builder() -> ListByKeysSessionsRequestBuilder {
        <ListByKeysSessionsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListByKeysSessionsRequestBuilder {
    args: Option<ListByKeysSessionsRequestArgs>,
}

impl ListByKeysSessionsRequestBuilder {
    pub fn args(mut self, value: ListByKeysSessionsRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListByKeysSessionsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](ListByKeysSessionsRequestBuilder::args)
    pub fn build(self) -> Result<ListByKeysSessionsRequest, BuildError> {
        Ok(ListByKeysSessionsRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

