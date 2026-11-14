pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListThreadsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<HashMap<String, serde_json::Value>>,
}

impl ListThreadsRequest {
    pub fn builder() -> ListThreadsRequestBuilder {
        <ListThreadsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListThreadsRequestBuilder {
    args: Option<HashMap<String, serde_json::Value>>,
}

impl ListThreadsRequestBuilder {
    pub fn args(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListThreadsRequest`].
    pub fn build(self) -> Result<ListThreadsRequest, BuildError> {
        Ok(ListThreadsRequest {
            args: self.args,
        })
    }
}

