pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ResetDemoWorkspaceRequest {
    #[serde(rename = "requestId")]
    #[serde(default)]
    pub request_id: String,
}

impl ResetDemoWorkspaceRequest {
    pub fn builder() -> ResetDemoWorkspaceRequestBuilder {
        <ResetDemoWorkspaceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResetDemoWorkspaceRequestBuilder {
    request_id: Option<String>,
}

impl ResetDemoWorkspaceRequestBuilder {
    pub fn request_id(mut self, value: impl Into<String>) -> Self {
        self.request_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ResetDemoWorkspaceRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`request_id`](ResetDemoWorkspaceRequestBuilder::request_id)
    pub fn build(self) -> Result<ResetDemoWorkspaceRequest, BuildError> {
        Ok(ResetDemoWorkspaceRequest {
            request_id: self.request_id.ok_or_else(|| BuildError::missing_field("request_id"))?,
        })
    }
}

