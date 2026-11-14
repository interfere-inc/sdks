pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RestoreProblemsRequestArgs {
    #[serde(rename = "problemId")]
    #[serde(default)]
    pub problem_id: String,
    #[serde(rename = "timelineEventId")]
    #[serde(default)]
    pub timeline_event_id: String,
}

impl RestoreProblemsRequestArgs {
    pub fn builder() -> RestoreProblemsRequestArgsBuilder {
        <RestoreProblemsRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RestoreProblemsRequestArgsBuilder {
    problem_id: Option<String>,
    timeline_event_id: Option<String>,
}

impl RestoreProblemsRequestArgsBuilder {
    pub fn problem_id(mut self, value: impl Into<String>) -> Self {
        self.problem_id = Some(value.into());
        self
    }

    pub fn timeline_event_id(mut self, value: impl Into<String>) -> Self {
        self.timeline_event_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RestoreProblemsRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`problem_id`](RestoreProblemsRequestArgsBuilder::problem_id)
    /// - [`timeline_event_id`](RestoreProblemsRequestArgsBuilder::timeline_event_id)
    pub fn build(self) -> Result<RestoreProblemsRequestArgs, BuildError> {
        Ok(RestoreProblemsRequestArgs {
            problem_id: self.problem_id.ok_or_else(|| BuildError::missing_field("problem_id"))?,
            timeline_event_id: self.timeline_event_id.ok_or_else(|| BuildError::missing_field("timeline_event_id"))?,
        })
    }
}
