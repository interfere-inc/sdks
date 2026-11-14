pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListConnectionsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<HashMap<String, serde_json::Value>>,
}

impl ListConnectionsRequest {
    pub fn builder() -> ListConnectionsRequestBuilder {
        <ListConnectionsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListConnectionsRequestBuilder {
    args: Option<HashMap<String, serde_json::Value>>,
}

impl ListConnectionsRequestBuilder {
    pub fn args(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListConnectionsRequest`].
    pub fn build(self) -> Result<ListConnectionsRequest, BuildError> {
        Ok(ListConnectionsRequest {
            args: self.args,
        })
    }
}

