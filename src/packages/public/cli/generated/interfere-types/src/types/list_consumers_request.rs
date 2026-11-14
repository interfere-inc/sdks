pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListConsumersRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<HashMap<String, serde_json::Value>>,
}

impl ListConsumersRequest {
    pub fn builder() -> ListConsumersRequestBuilder {
        <ListConsumersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListConsumersRequestBuilder {
    args: Option<HashMap<String, serde_json::Value>>,
}

impl ListConsumersRequestBuilder {
    pub fn args(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListConsumersRequest`].
    pub fn build(self) -> Result<ListConsumersRequest, BuildError> {
        Ok(ListConsumersRequest {
            args: self.args,
        })
    }
}

