pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListRequestsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<HashMap<String, serde_json::Value>>,
}

impl ListRequestsRequest {
    pub fn builder() -> ListRequestsRequestBuilder {
        <ListRequestsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListRequestsRequestBuilder {
    args: Option<HashMap<String, serde_json::Value>>,
}

impl ListRequestsRequestBuilder {
    pub fn args(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListRequestsRequest`].
    pub fn build(self) -> Result<ListRequestsRequest, BuildError> {
        Ok(ListRequestsRequest {
            args: self.args,
        })
    }
}

