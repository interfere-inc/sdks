pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CurrentWorkspacesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<HashMap<String, serde_json::Value>>,
}

impl CurrentWorkspacesRequest {
    pub fn builder() -> CurrentWorkspacesRequestBuilder {
        <CurrentWorkspacesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CurrentWorkspacesRequestBuilder {
    args: Option<HashMap<String, serde_json::Value>>,
}

impl CurrentWorkspacesRequestBuilder {
    pub fn args(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CurrentWorkspacesRequest`].
    pub fn build(self) -> Result<CurrentWorkspacesRequest, BuildError> {
        Ok(CurrentWorkspacesRequest {
            args: self.args,
        })
    }
}

