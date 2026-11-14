pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateAutoAssignProblemsWorkspacesRequestArgs {
    #[serde(rename = "autoAssignProblems")]
    #[serde(default)]
    pub auto_assign_problems: bool,
}

impl UpdateAutoAssignProblemsWorkspacesRequestArgs {
    pub fn builder() -> UpdateAutoAssignProblemsWorkspacesRequestArgsBuilder {
        <UpdateAutoAssignProblemsWorkspacesRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateAutoAssignProblemsWorkspacesRequestArgsBuilder {
    auto_assign_problems: Option<bool>,
}

impl UpdateAutoAssignProblemsWorkspacesRequestArgsBuilder {
    pub fn auto_assign_problems(mut self, value: bool) -> Self {
        self.auto_assign_problems = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateAutoAssignProblemsWorkspacesRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`auto_assign_problems`](UpdateAutoAssignProblemsWorkspacesRequestArgsBuilder::auto_assign_problems)
    pub fn build(self) -> Result<UpdateAutoAssignProblemsWorkspacesRequestArgs, BuildError> {
        Ok(UpdateAutoAssignProblemsWorkspacesRequestArgs {
            auto_assign_problems: self.auto_assign_problems.ok_or_else(|| BuildError::missing_field("auto_assign_problems"))?,
        })
    }
}
