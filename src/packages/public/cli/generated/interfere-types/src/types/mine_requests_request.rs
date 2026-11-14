pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MineRequestsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<HashMap<String, serde_json::Value>>,
}

impl MineRequestsRequest {
    pub fn builder() -> MineRequestsRequestBuilder {
        <MineRequestsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MineRequestsRequestBuilder {
    args: Option<HashMap<String, serde_json::Value>>,
}

impl MineRequestsRequestBuilder {
    pub fn args(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MineRequestsRequest`].
    pub fn build(self) -> Result<MineRequestsRequest, BuildError> {
        Ok(MineRequestsRequest {
            args: self.args,
        })
    }
}

