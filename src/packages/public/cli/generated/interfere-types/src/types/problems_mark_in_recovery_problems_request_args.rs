pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MarkInRecoveryProblemsRequestArgs {
    #[serde(rename = "problemId")]
    #[serde(default)]
    pub problem_id: String,
    #[serde(rename = "timelineEventId")]
    #[serde(default)]
    pub timeline_event_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fix: Option<MarkInRecoveryProblemsRequestArgsFix>,
}

impl MarkInRecoveryProblemsRequestArgs {
    pub fn builder() -> MarkInRecoveryProblemsRequestArgsBuilder {
        <MarkInRecoveryProblemsRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MarkInRecoveryProblemsRequestArgsBuilder {
    problem_id: Option<String>,
    timeline_event_id: Option<String>,
    fix: Option<MarkInRecoveryProblemsRequestArgsFix>,
}

impl MarkInRecoveryProblemsRequestArgsBuilder {
    pub fn problem_id(mut self, value: impl Into<String>) -> Self {
        self.problem_id = Some(value.into());
        self
    }

    pub fn timeline_event_id(mut self, value: impl Into<String>) -> Self {
        self.timeline_event_id = Some(value.into());
        self
    }

    pub fn fix(mut self, value: MarkInRecoveryProblemsRequestArgsFix) -> Self {
        self.fix = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MarkInRecoveryProblemsRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`problem_id`](MarkInRecoveryProblemsRequestArgsBuilder::problem_id)
    /// - [`timeline_event_id`](MarkInRecoveryProblemsRequestArgsBuilder::timeline_event_id)
    pub fn build(self) -> Result<MarkInRecoveryProblemsRequestArgs, BuildError> {
        Ok(MarkInRecoveryProblemsRequestArgs {
            problem_id: self.problem_id.ok_or_else(|| BuildError::missing_field("problem_id"))?,
            timeline_event_id: self.timeline_event_id.ok_or_else(|| BuildError::missing_field("timeline_event_id"))?,
            fix: self.fix,
        })
    }
}
