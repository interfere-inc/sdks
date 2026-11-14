pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct NavigableProblemsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<HashMap<String, serde_json::Value>>,
}

impl NavigableProblemsRequest {
    pub fn builder() -> NavigableProblemsRequestBuilder {
        <NavigableProblemsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NavigableProblemsRequestBuilder {
    args: Option<HashMap<String, serde_json::Value>>,
}

impl NavigableProblemsRequestBuilder {
    pub fn args(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`NavigableProblemsRequest`].
    pub fn build(self) -> Result<NavigableProblemsRequest, BuildError> {
        Ok(NavigableProblemsRequest {
            args: self.args,
        })
    }
}

