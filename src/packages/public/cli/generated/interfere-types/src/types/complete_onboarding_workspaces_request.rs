pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CompleteOnboardingWorkspacesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<serde_json::Value>,
}

impl CompleteOnboardingWorkspacesRequest {
    pub fn builder() -> CompleteOnboardingWorkspacesRequestBuilder {
        <CompleteOnboardingWorkspacesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompleteOnboardingWorkspacesRequestBuilder {
    args: Option<serde_json::Value>,
}

impl CompleteOnboardingWorkspacesRequestBuilder {
    pub fn args(mut self, value: serde_json::Value) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CompleteOnboardingWorkspacesRequest`].
    pub fn build(self) -> Result<CompleteOnboardingWorkspacesRequest, BuildError> {
        Ok(CompleteOnboardingWorkspacesRequest {
            args: self.args,
        })
    }
}

