pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateAssigneeProblemsRequestArgs {
    #[serde(rename = "problemId")]
    #[serde(default)]
    pub problem_id: String,
    #[serde(rename = "assigneeUserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee_user_id: Option<String>,
    #[serde(rename = "timelineEventId")]
    #[serde(default)]
    pub timeline_event_id: String,
}

impl UpdateAssigneeProblemsRequestArgs {
    pub fn builder() -> UpdateAssigneeProblemsRequestArgsBuilder {
        <UpdateAssigneeProblemsRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateAssigneeProblemsRequestArgsBuilder {
    problem_id: Option<String>,
    assignee_user_id: Option<String>,
    timeline_event_id: Option<String>,
}

impl UpdateAssigneeProblemsRequestArgsBuilder {
    pub fn problem_id(mut self, value: impl Into<String>) -> Self {
        self.problem_id = Some(value.into());
        self
    }

    pub fn assignee_user_id(mut self, value: impl Into<String>) -> Self {
        self.assignee_user_id = Some(value.into());
        self
    }

    pub fn timeline_event_id(mut self, value: impl Into<String>) -> Self {
        self.timeline_event_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateAssigneeProblemsRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`problem_id`](UpdateAssigneeProblemsRequestArgsBuilder::problem_id)
    /// - [`timeline_event_id`](UpdateAssigneeProblemsRequestArgsBuilder::timeline_event_id)
    pub fn build(self) -> Result<UpdateAssigneeProblemsRequestArgs, BuildError> {
        Ok(UpdateAssigneeProblemsRequestArgs {
            problem_id: self.problem_id.ok_or_else(|| BuildError::missing_field("problem_id"))?,
            assignee_user_id: self.assignee_user_id,
            timeline_event_id: self.timeline_event_id.ok_or_else(|| BuildError::missing_field("timeline_event_id"))?,
        })
    }
}
