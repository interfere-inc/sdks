pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DestroyWorkspacesRequest {
    #[serde(default)]
    pub args: HashMap<String, serde_json::Value>,
}

impl DestroyWorkspacesRequest {
    pub fn builder() -> DestroyWorkspacesRequestBuilder {
        <DestroyWorkspacesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DestroyWorkspacesRequestBuilder {
    args: Option<HashMap<String, serde_json::Value>>,
}

impl DestroyWorkspacesRequestBuilder {
    pub fn args(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DestroyWorkspacesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](DestroyWorkspacesRequestBuilder::args)
    pub fn build(self) -> Result<DestroyWorkspacesRequest, BuildError> {
        Ok(DestroyWorkspacesRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

