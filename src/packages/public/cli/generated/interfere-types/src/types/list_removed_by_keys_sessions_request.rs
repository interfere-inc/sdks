pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListRemovedByKeysSessionsRequest {
    #[serde(default)]
    pub args: ListRemovedByKeysSessionsRequestArgs,
}

impl ListRemovedByKeysSessionsRequest {
    pub fn builder() -> ListRemovedByKeysSessionsRequestBuilder {
        <ListRemovedByKeysSessionsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListRemovedByKeysSessionsRequestBuilder {
    args: Option<ListRemovedByKeysSessionsRequestArgs>,
}

impl ListRemovedByKeysSessionsRequestBuilder {
    pub fn args(mut self, value: ListRemovedByKeysSessionsRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListRemovedByKeysSessionsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](ListRemovedByKeysSessionsRequestBuilder::args)
    pub fn build(self) -> Result<ListRemovedByKeysSessionsRequest, BuildError> {
        Ok(ListRemovedByKeysSessionsRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

