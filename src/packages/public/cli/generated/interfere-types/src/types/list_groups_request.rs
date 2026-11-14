pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListGroupsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<HashMap<String, serde_json::Value>>,
}

impl ListGroupsRequest {
    pub fn builder() -> ListGroupsRequestBuilder {
        <ListGroupsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListGroupsRequestBuilder {
    args: Option<HashMap<String, serde_json::Value>>,
}

impl ListGroupsRequestBuilder {
    pub fn args(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListGroupsRequest`].
    pub fn build(self) -> Result<ListGroupsRequest, BuildError> {
        Ok(ListGroupsRequest {
            args: self.args,
        })
    }
}

