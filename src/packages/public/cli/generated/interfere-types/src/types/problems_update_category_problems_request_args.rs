pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateCategoryProblemsRequestArgs {
    #[serde(rename = "problemId")]
    #[serde(default)]
    pub problem_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<UpdateCategoryProblemsRequestArgsCategory>,
    #[serde(rename = "timelineEventId")]
    #[serde(default)]
    pub timeline_event_id: String,
}

impl UpdateCategoryProblemsRequestArgs {
    pub fn builder() -> UpdateCategoryProblemsRequestArgsBuilder {
        <UpdateCategoryProblemsRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateCategoryProblemsRequestArgsBuilder {
    problem_id: Option<String>,
    category: Option<UpdateCategoryProblemsRequestArgsCategory>,
    timeline_event_id: Option<String>,
}

impl UpdateCategoryProblemsRequestArgsBuilder {
    pub fn problem_id(mut self, value: impl Into<String>) -> Self {
        self.problem_id = Some(value.into());
        self
    }

    pub fn category(mut self, value: UpdateCategoryProblemsRequestArgsCategory) -> Self {
        self.category = Some(value);
        self
    }

    pub fn timeline_event_id(mut self, value: impl Into<String>) -> Self {
        self.timeline_event_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateCategoryProblemsRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`problem_id`](UpdateCategoryProblemsRequestArgsBuilder::problem_id)
    /// - [`timeline_event_id`](UpdateCategoryProblemsRequestArgsBuilder::timeline_event_id)
    pub fn build(self) -> Result<UpdateCategoryProblemsRequestArgs, BuildError> {
        Ok(UpdateCategoryProblemsRequestArgs {
            problem_id: self.problem_id.ok_or_else(|| BuildError::missing_field("problem_id"))?,
            category: self.category,
            timeline_event_id: self.timeline_event_id.ok_or_else(|| BuildError::missing_field("timeline_event_id"))?,
        })
    }
}
