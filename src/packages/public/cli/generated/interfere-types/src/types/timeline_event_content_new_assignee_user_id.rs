pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TimelineEventContentNewAssigneeUserId {
    #[serde(rename = "newAssigneeUserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_assignee_user_id: Option<String>,
    #[serde(rename = "prevAssigneeUserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_assignee_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggestion: Option<TimelineEventContentNewAssigneeUserIdSuggestion>,
}

impl TimelineEventContentNewAssigneeUserId {
    pub fn builder() -> TimelineEventContentNewAssigneeUserIdBuilder {
        <TimelineEventContentNewAssigneeUserIdBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentNewAssigneeUserIdBuilder {
    new_assignee_user_id: Option<String>,
    prev_assignee_user_id: Option<String>,
    suggestion: Option<TimelineEventContentNewAssigneeUserIdSuggestion>,
}

impl TimelineEventContentNewAssigneeUserIdBuilder {
    pub fn new_assignee_user_id(mut self, value: impl Into<String>) -> Self {
        self.new_assignee_user_id = Some(value.into());
        self
    }

    pub fn prev_assignee_user_id(mut self, value: impl Into<String>) -> Self {
        self.prev_assignee_user_id = Some(value.into());
        self
    }

    pub fn suggestion(mut self, value: TimelineEventContentNewAssigneeUserIdSuggestion) -> Self {
        self.suggestion = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentNewAssigneeUserId`].
    pub fn build(self) -> Result<TimelineEventContentNewAssigneeUserId, BuildError> {
        Ok(TimelineEventContentNewAssigneeUserId {
            new_assignee_user_id: self.new_assignee_user_id,
            prev_assignee_user_id: self.prev_assignee_user_id,
            suggestion: self.suggestion,
        })
    }
}
