pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CorrectRootCauseResponse {
    #[serde(rename = "problemId")]
    #[serde(default)]
    pub problem_id: String,
    #[serde(rename = "eventId")]
    #[serde(default)]
    pub event_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supersedes: Option<String>,
    #[serde(default)]
    pub completed: bool,
}

impl CorrectRootCauseResponse {
    pub fn builder() -> CorrectRootCauseResponseBuilder {
        <CorrectRootCauseResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CorrectRootCauseResponseBuilder {
    problem_id: Option<String>,
    event_id: Option<String>,
    supersedes: Option<String>,
    completed: Option<bool>,
}

impl CorrectRootCauseResponseBuilder {
    pub fn problem_id(mut self, value: impl Into<String>) -> Self {
        self.problem_id = Some(value.into());
        self
    }

    pub fn event_id(mut self, value: impl Into<String>) -> Self {
        self.event_id = Some(value.into());
        self
    }

    pub fn supersedes(mut self, value: impl Into<String>) -> Self {
        self.supersedes = Some(value.into());
        self
    }

    pub fn completed(mut self, value: bool) -> Self {
        self.completed = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CorrectRootCauseResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`problem_id`](CorrectRootCauseResponseBuilder::problem_id)
    /// - [`event_id`](CorrectRootCauseResponseBuilder::event_id)
    /// - [`completed`](CorrectRootCauseResponseBuilder::completed)
    pub fn build(self) -> Result<CorrectRootCauseResponse, BuildError> {
        Ok(CorrectRootCauseResponse {
            problem_id: self.problem_id.ok_or_else(|| BuildError::missing_field("problem_id"))?,
            event_id: self.event_id.ok_or_else(|| BuildError::missing_field("event_id"))?,
            supersedes: self.supersedes,
            completed: self.completed.ok_or_else(|| BuildError::missing_field("completed"))?,
        })
    }
}
