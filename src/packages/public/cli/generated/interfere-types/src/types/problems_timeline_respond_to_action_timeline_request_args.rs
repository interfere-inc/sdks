pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RespondToActionTimelineRequestArgs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<RespondToActionTimelineRequestArgsResolution>,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "problemId")]
    #[serde(default)]
    pub problem_id: String,
    #[serde(rename = "selectedOptionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_option_id: Option<String>,
    #[serde(rename = "selectedOptionIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_option_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answer: Option<String>,
}

impl RespondToActionTimelineRequestArgs {
    pub fn builder() -> RespondToActionTimelineRequestArgsBuilder {
        <RespondToActionTimelineRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RespondToActionTimelineRequestArgsBuilder {
    resolution: Option<RespondToActionTimelineRequestArgsResolution>,
    id: Option<String>,
    problem_id: Option<String>,
    selected_option_id: Option<String>,
    selected_option_ids: Option<Vec<String>>,
    answer: Option<String>,
}

impl RespondToActionTimelineRequestArgsBuilder {
    pub fn resolution(mut self, value: RespondToActionTimelineRequestArgsResolution) -> Self {
        self.resolution = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn problem_id(mut self, value: impl Into<String>) -> Self {
        self.problem_id = Some(value.into());
        self
    }

    pub fn selected_option_id(mut self, value: impl Into<String>) -> Self {
        self.selected_option_id = Some(value.into());
        self
    }

    pub fn selected_option_ids(mut self, value: Vec<String>) -> Self {
        self.selected_option_ids = Some(value);
        self
    }

    pub fn answer(mut self, value: impl Into<String>) -> Self {
        self.answer = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RespondToActionTimelineRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](RespondToActionTimelineRequestArgsBuilder::id)
    /// - [`problem_id`](RespondToActionTimelineRequestArgsBuilder::problem_id)
    pub fn build(self) -> Result<RespondToActionTimelineRequestArgs, BuildError> {
        Ok(RespondToActionTimelineRequestArgs {
            resolution: self.resolution,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            problem_id: self.problem_id.ok_or_else(|| BuildError::missing_field("problem_id"))?,
            selected_option_id: self.selected_option_id,
            selected_option_ids: self.selected_option_ids,
            answer: self.answer,
        })
    }
}
