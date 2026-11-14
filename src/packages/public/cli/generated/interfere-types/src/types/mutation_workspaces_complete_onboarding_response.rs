pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationWorkspacesCompleteOnboardingResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationWorkspacesCompleteOnboardingResponse {
    pub fn builder() -> MutationWorkspacesCompleteOnboardingResponseBuilder {
        <MutationWorkspacesCompleteOnboardingResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationWorkspacesCompleteOnboardingResponseBuilder {
    success: Option<bool>,
}

impl MutationWorkspacesCompleteOnboardingResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationWorkspacesCompleteOnboardingResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationWorkspacesCompleteOnboardingResponseBuilder::success)
    pub fn build(self) -> Result<MutationWorkspacesCompleteOnboardingResponse, BuildError> {
        Ok(MutationWorkspacesCompleteOnboardingResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
