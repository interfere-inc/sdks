pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListProblemsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<HashMap<String, serde_json::Value>>,
}

impl ListProblemsRequest {
    pub fn builder() -> ListProblemsRequestBuilder {
        <ListProblemsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListProblemsRequestBuilder {
    args: Option<HashMap<String, serde_json::Value>>,
}

impl ListProblemsRequestBuilder {
    pub fn args(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListProblemsRequest`].
    pub fn build(self) -> Result<ListProblemsRequest, BuildError> {
        Ok(ListProblemsRequest {
            args: self.args,
        })
    }
}

