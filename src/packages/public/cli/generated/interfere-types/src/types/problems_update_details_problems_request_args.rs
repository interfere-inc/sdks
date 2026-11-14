pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateDetailsProblemsRequestArgs {
    #[serde(rename = "problemId")]
    #[serde(default)]
    pub problem_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "titleTimelineEventId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_timeline_event_id: Option<String>,
    #[serde(rename = "descriptionTimelineEventId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_timeline_event_id: Option<String>,
}

impl UpdateDetailsProblemsRequestArgs {
    pub fn builder() -> UpdateDetailsProblemsRequestArgsBuilder {
        <UpdateDetailsProblemsRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateDetailsProblemsRequestArgsBuilder {
    problem_id: Option<String>,
    title: Option<String>,
    description: Option<String>,
    title_timeline_event_id: Option<String>,
    description_timeline_event_id: Option<String>,
}

impl UpdateDetailsProblemsRequestArgsBuilder {
    pub fn problem_id(mut self, value: impl Into<String>) -> Self {
        self.problem_id = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn title_timeline_event_id(mut self, value: impl Into<String>) -> Self {
        self.title_timeline_event_id = Some(value.into());
        self
    }

    pub fn description_timeline_event_id(mut self, value: impl Into<String>) -> Self {
        self.description_timeline_event_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateDetailsProblemsRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`problem_id`](UpdateDetailsProblemsRequestArgsBuilder::problem_id)
    pub fn build(self) -> Result<UpdateDetailsProblemsRequestArgs, BuildError> {
        Ok(UpdateDetailsProblemsRequestArgs {
            problem_id: self.problem_id.ok_or_else(|| BuildError::missing_field("problem_id"))?,
            title: self.title,
            description: self.description,
            title_timeline_event_id: self.title_timeline_event_id,
            description_timeline_event_id: self.description_timeline_event_id,
        })
    }
}
