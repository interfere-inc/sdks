pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RulesNetworkRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<HashMap<String, serde_json::Value>>,
}

impl RulesNetworkRequest {
    pub fn builder() -> RulesNetworkRequestBuilder {
        <RulesNetworkRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RulesNetworkRequestBuilder {
    args: Option<HashMap<String, serde_json::Value>>,
}

impl RulesNetworkRequestBuilder {
    pub fn args(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RulesNetworkRequest`].
    pub fn build(self) -> Result<RulesNetworkRequest, BuildError> {
        Ok(RulesNetworkRequest {
            args: self.args,
        })
    }
}

