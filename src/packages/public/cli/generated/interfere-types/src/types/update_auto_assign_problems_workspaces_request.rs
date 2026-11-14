pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateAutoAssignProblemsWorkspacesRequest {
    #[serde(default)]
    pub args: UpdateAutoAssignProblemsWorkspacesRequestArgs,
}

impl UpdateAutoAssignProblemsWorkspacesRequest {
    pub fn builder() -> UpdateAutoAssignProblemsWorkspacesRequestBuilder {
        <UpdateAutoAssignProblemsWorkspacesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateAutoAssignProblemsWorkspacesRequestBuilder {
    args: Option<UpdateAutoAssignProblemsWorkspacesRequestArgs>,
}

impl UpdateAutoAssignProblemsWorkspacesRequestBuilder {
    pub fn args(mut self, value: UpdateAutoAssignProblemsWorkspacesRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateAutoAssignProblemsWorkspacesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](UpdateAutoAssignProblemsWorkspacesRequestBuilder::args)
    pub fn build(self) -> Result<UpdateAutoAssignProblemsWorkspacesRequest, BuildError> {
        Ok(UpdateAutoAssignProblemsWorkspacesRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

