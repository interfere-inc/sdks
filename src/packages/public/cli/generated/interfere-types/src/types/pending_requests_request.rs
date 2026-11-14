pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PendingRequestsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<HashMap<String, serde_json::Value>>,
}

impl PendingRequestsRequest {
    pub fn builder() -> PendingRequestsRequestBuilder {
        <PendingRequestsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PendingRequestsRequestBuilder {
    args: Option<HashMap<String, serde_json::Value>>,
}

impl PendingRequestsRequestBuilder {
    pub fn args(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PendingRequestsRequest`].
    pub fn build(self) -> Result<PendingRequestsRequest, BuildError> {
        Ok(PendingRequestsRequest {
            args: self.args,
        })
    }
}

