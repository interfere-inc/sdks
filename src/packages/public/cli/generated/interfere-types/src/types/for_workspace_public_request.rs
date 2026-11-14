pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ForWorkspacePublicRequest {
    #[serde(default)]
    pub args: HashMap<String, serde_json::Value>,
}

impl ForWorkspacePublicRequest {
    pub fn builder() -> ForWorkspacePublicRequestBuilder {
        <ForWorkspacePublicRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ForWorkspacePublicRequestBuilder {
    args: Option<HashMap<String, serde_json::Value>>,
}

impl ForWorkspacePublicRequestBuilder {
    pub fn args(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ForWorkspacePublicRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](ForWorkspacePublicRequestBuilder::args)
    pub fn build(self) -> Result<ForWorkspacePublicRequest, BuildError> {
        Ok(ForWorkspacePublicRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

