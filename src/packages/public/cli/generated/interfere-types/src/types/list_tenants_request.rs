pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListTenantsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<HashMap<String, serde_json::Value>>,
}

impl ListTenantsRequest {
    pub fn builder() -> ListTenantsRequestBuilder {
        <ListTenantsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListTenantsRequestBuilder {
    args: Option<HashMap<String, serde_json::Value>>,
}

impl ListTenantsRequestBuilder {
    pub fn args(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListTenantsRequest`].
    pub fn build(self) -> Result<ListTenantsRequest, BuildError> {
        Ok(ListTenantsRequest {
            args: self.args,
        })
    }
}

