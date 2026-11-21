pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNewAssigneeGroupId {
    #[serde(rename = "newAssigneeGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_assignee_group_id: Option<String>,
    #[serde(rename = "newAssigneeUserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_assignee_user_id: Option<String>,
    #[serde(rename = "prevAssigneeGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_assignee_group_id: Option<String>,
    #[serde(rename = "prevAssigneeUserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_assignee_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggestion: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNewAssigneeGroupIdSuggestion>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNewAssigneeGroupId {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNewAssigneeGroupIdBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNewAssigneeGroupIdBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNewAssigneeGroupIdBuilder {
    new_assignee_group_id: Option<String>,
    new_assignee_user_id: Option<String>,
    prev_assignee_group_id: Option<String>,
    prev_assignee_user_id: Option<String>,
    suggestion: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNewAssigneeGroupIdSuggestion>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNewAssigneeGroupIdBuilder {
    pub fn new_assignee_group_id(mut self, value: impl Into<String>) -> Self {
        self.new_assignee_group_id = Some(value.into());
        self
    }

    pub fn new_assignee_user_id(mut self, value: impl Into<String>) -> Self {
        self.new_assignee_user_id = Some(value.into());
        self
    }

    pub fn prev_assignee_group_id(mut self, value: impl Into<String>) -> Self {
        self.prev_assignee_group_id = Some(value.into());
        self
    }

    pub fn prev_assignee_user_id(mut self, value: impl Into<String>) -> Self {
        self.prev_assignee_user_id = Some(value.into());
        self
    }

    pub fn suggestion(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNewAssigneeGroupIdSuggestion) -> Self {
        self.suggestion = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNewAssigneeGroupId`].
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNewAssigneeGroupId, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNewAssigneeGroupId {
            new_assignee_group_id: self.new_assignee_group_id,
            new_assignee_user_id: self.new_assignee_user_id,
            prev_assignee_group_id: self.prev_assignee_group_id,
            prev_assignee_user_id: self.prev_assignee_user_id,
            suggestion: self.suggestion,
        })
    }
}
