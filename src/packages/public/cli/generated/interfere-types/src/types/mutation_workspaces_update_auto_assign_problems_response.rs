pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationWorkspacesUpdateAutoAssignProblemsResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationWorkspacesUpdateAutoAssignProblemsResponse {
    pub fn builder() -> MutationWorkspacesUpdateAutoAssignProblemsResponseBuilder {
        <MutationWorkspacesUpdateAutoAssignProblemsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationWorkspacesUpdateAutoAssignProblemsResponseBuilder {
    success: Option<bool>,
}

impl MutationWorkspacesUpdateAutoAssignProblemsResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationWorkspacesUpdateAutoAssignProblemsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationWorkspacesUpdateAutoAssignProblemsResponseBuilder::success)
    pub fn build(self) -> Result<MutationWorkspacesUpdateAutoAssignProblemsResponse, BuildError> {
        Ok(MutationWorkspacesUpdateAutoAssignProblemsResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
