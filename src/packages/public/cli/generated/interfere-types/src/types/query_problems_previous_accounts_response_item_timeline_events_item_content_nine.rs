pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNine {
        StringToValueMap(HashMap<String, serde_json::Value>),

        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNineConfirmationId(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNineConfirmationId),

        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNineAffectedUsers(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNineAffectedUsers),
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNine {
    pub fn is_string_to_value_map(&self) -> bool {
        matches!(self, Self::StringToValueMap(_))
    }

    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_nine_confirmation_id(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNineConfirmationId(_))
    }

    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_nine_affected_users(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNineAffectedUsers(_))
    }


    pub fn as_string_to_value_map(&self) -> Option<&HashMap<String, serde_json::Value>> {
        match self {
                    Self::StringToValueMap(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_string_to_value_map(self) -> Option<HashMap<String, serde_json::Value>> {
        match self {
                    Self::StringToValueMap(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_content_nine_confirmation_id(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNineConfirmationId> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNineConfirmationId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_content_nine_confirmation_id(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNineConfirmationId> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNineConfirmationId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_content_nine_affected_users(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNineAffectedUsers> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNineAffectedUsers(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_content_nine_affected_users(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNineAffectedUsers> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNineAffectedUsers(value) => Some(value),
                    _ => None,
                }
    }
}
