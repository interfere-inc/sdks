pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentNewAssigneeUserId {
    #[serde(rename = "newAssigneeUserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_assignee_user_id: Option<String>,
    #[serde(rename = "prevAssigneeUserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_assignee_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggestion: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentNewAssigneeUserIdSuggestion>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentNewAssigneeUserId {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentNewAssigneeUserIdBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentNewAssigneeUserIdBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentNewAssigneeUserIdBuilder {
    new_assignee_user_id: Option<String>,
    prev_assignee_user_id: Option<String>,
    suggestion: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentNewAssigneeUserIdSuggestion>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentNewAssigneeUserIdBuilder {
    pub fn new_assignee_user_id(mut self, value: impl Into<String>) -> Self {
        self.new_assignee_user_id = Some(value.into());
        self
    }

    pub fn prev_assignee_user_id(mut self, value: impl Into<String>) -> Self {
        self.prev_assignee_user_id = Some(value.into());
        self
    }

    pub fn suggestion(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentNewAssigneeUserIdSuggestion) -> Self {
        self.suggestion = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentNewAssigneeUserId`].
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentNewAssigneeUserId, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentNewAssigneeUserId {
            new_assignee_user_id: self.new_assignee_user_id,
            prev_assignee_user_id: self.prev_assignee_user_id,
            suggestion: self.suggestion,
        })
    }
}
