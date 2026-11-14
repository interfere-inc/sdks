pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdatePriorityProblemsRequestArgs {
    #[serde(rename = "problemId")]
    #[serde(default)]
    pub problem_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<UpdatePriorityProblemsRequestArgsPriority>,
    #[serde(rename = "timelineEventId")]
    #[serde(default)]
    pub timeline_event_id: String,
}

impl UpdatePriorityProblemsRequestArgs {
    pub fn builder() -> UpdatePriorityProblemsRequestArgsBuilder {
        <UpdatePriorityProblemsRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdatePriorityProblemsRequestArgsBuilder {
    problem_id: Option<String>,
    priority: Option<UpdatePriorityProblemsRequestArgsPriority>,
    timeline_event_id: Option<String>,
}

impl UpdatePriorityProblemsRequestArgsBuilder {
    pub fn problem_id(mut self, value: impl Into<String>) -> Self {
        self.problem_id = Some(value.into());
        self
    }

    pub fn priority(mut self, value: UpdatePriorityProblemsRequestArgsPriority) -> Self {
        self.priority = Some(value);
        self
    }

    pub fn timeline_event_id(mut self, value: impl Into<String>) -> Self {
        self.timeline_event_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdatePriorityProblemsRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`problem_id`](UpdatePriorityProblemsRequestArgsBuilder::problem_id)
    /// - [`timeline_event_id`](UpdatePriorityProblemsRequestArgsBuilder::timeline_event_id)
    pub fn build(self) -> Result<UpdatePriorityProblemsRequestArgs, BuildError> {
        Ok(UpdatePriorityProblemsRequestArgs {
            problem_id: self.problem_id.ok_or_else(|| BuildError::missing_field("problem_id"))?,
            priority: self.priority,
            timeline_event_id: self.timeline_event_id.ok_or_else(|| BuildError::missing_field("timeline_event_id"))?,
        })
    }
}
