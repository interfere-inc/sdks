pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ForInboxEnvironmentsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<HashMap<String, serde_json::Value>>,
}

impl ForInboxEnvironmentsRequest {
    pub fn builder() -> ForInboxEnvironmentsRequestBuilder {
        <ForInboxEnvironmentsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ForInboxEnvironmentsRequestBuilder {
    args: Option<HashMap<String, serde_json::Value>>,
}

impl ForInboxEnvironmentsRequestBuilder {
    pub fn args(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ForInboxEnvironmentsRequest`].
    pub fn build(self) -> Result<ForInboxEnvironmentsRequest, BuildError> {
        Ok(ForInboxEnvironmentsRequest {
            args: self.args,
        })
    }
}

