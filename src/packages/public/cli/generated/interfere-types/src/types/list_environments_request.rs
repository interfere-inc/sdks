pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListEnvironmentsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<HashMap<String, serde_json::Value>>,
}

impl ListEnvironmentsRequest {
    pub fn builder() -> ListEnvironmentsRequestBuilder {
        <ListEnvironmentsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListEnvironmentsRequestBuilder {
    args: Option<HashMap<String, serde_json::Value>>,
}

impl ListEnvironmentsRequestBuilder {
    pub fn args(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListEnvironmentsRequest`].
    pub fn build(self) -> Result<ListEnvironmentsRequest, BuildError> {
        Ok(ListEnvironmentsRequest {
            args: self.args,
        })
    }
}

