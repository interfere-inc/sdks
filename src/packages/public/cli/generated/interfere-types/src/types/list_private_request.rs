pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListPrivateRequest {
    #[serde(default)]
    pub args: HashMap<String, serde_json::Value>,
}

impl ListPrivateRequest {
    pub fn builder() -> ListPrivateRequestBuilder {
        <ListPrivateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPrivateRequestBuilder {
    args: Option<HashMap<String, serde_json::Value>>,
}

impl ListPrivateRequestBuilder {
    pub fn args(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListPrivateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](ListPrivateRequestBuilder::args)
    pub fn build(self) -> Result<ListPrivateRequest, BuildError> {
        Ok(ListPrivateRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

