pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ActionRequestsProblemsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<HashMap<String, serde_json::Value>>,
}

impl ActionRequestsProblemsRequest {
    pub fn builder() -> ActionRequestsProblemsRequestBuilder {
        <ActionRequestsProblemsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ActionRequestsProblemsRequestBuilder {
    args: Option<HashMap<String, serde_json::Value>>,
}

impl ActionRequestsProblemsRequestBuilder {
    pub fn args(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ActionRequestsProblemsRequest`].
    pub fn build(self) -> Result<ActionRequestsProblemsRequest, BuildError> {
        Ok(ActionRequestsProblemsRequest {
            args: self.args,
        })
    }
}

