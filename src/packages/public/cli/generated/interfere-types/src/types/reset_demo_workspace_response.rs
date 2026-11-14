pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ResetDemoWorkspaceResponse {
    #[serde(default)]
    pub started: String,
}

impl ResetDemoWorkspaceResponse {
    pub fn builder() -> ResetDemoWorkspaceResponseBuilder {
        <ResetDemoWorkspaceResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResetDemoWorkspaceResponseBuilder {
    started: Option<String>,
}

impl ResetDemoWorkspaceResponseBuilder {
    pub fn started(mut self, value: impl Into<String>) -> Self {
        self.started = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ResetDemoWorkspaceResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`started`](ResetDemoWorkspaceResponseBuilder::started)
    pub fn build(self) -> Result<ResetDemoWorkspaceResponse, BuildError> {
        Ok(ResetDemoWorkspaceResponse {
            started: self.started.ok_or_else(|| BuildError::missing_field("started"))?,
        })
    }
}
