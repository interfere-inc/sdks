pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DismissProblemsRequestArgs {
    #[serde(rename = "problemId")]
    #[serde(default)]
    pub problem_id: String,
    #[serde(rename = "timelineEventId")]
    #[serde(default)]
    pub timeline_event_id: String,
}

impl DismissProblemsRequestArgs {
    pub fn builder() -> DismissProblemsRequestArgsBuilder {
        <DismissProblemsRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DismissProblemsRequestArgsBuilder {
    problem_id: Option<String>,
    timeline_event_id: Option<String>,
}

impl DismissProblemsRequestArgsBuilder {
    pub fn problem_id(mut self, value: impl Into<String>) -> Self {
        self.problem_id = Some(value.into());
        self
    }

    pub fn timeline_event_id(mut self, value: impl Into<String>) -> Self {
        self.timeline_event_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DismissProblemsRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`problem_id`](DismissProblemsRequestArgsBuilder::problem_id)
    /// - [`timeline_event_id`](DismissProblemsRequestArgsBuilder::timeline_event_id)
    pub fn build(self) -> Result<DismissProblemsRequestArgs, BuildError> {
        Ok(DismissProblemsRequestArgs {
            problem_id: self.problem_id.ok_or_else(|| BuildError::missing_field("problem_id"))?,
            timeline_event_id: self.timeline_event_id.ok_or_else(|| BuildError::missing_field("timeline_event_id"))?,
        })
    }
}
