pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListTiersRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<HashMap<String, serde_json::Value>>,
}

impl ListTiersRequest {
    pub fn builder() -> ListTiersRequestBuilder {
        <ListTiersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListTiersRequestBuilder {
    args: Option<HashMap<String, serde_json::Value>>,
}

impl ListTiersRequestBuilder {
    pub fn args(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListTiersRequest`].
    pub fn build(self) -> Result<ListTiersRequest, BuildError> {
        Ok(ListTiersRequest {
            args: self.args,
        })
    }
}

