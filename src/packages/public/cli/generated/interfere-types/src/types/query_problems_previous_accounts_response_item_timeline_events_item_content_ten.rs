pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTen {
        StringToValueMap(HashMap<String, serde_json::Value>),

        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTenConfirmationId(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTenConfirmationId),

        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTenAffectedUsers(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTenAffectedUsers),
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTen {
    pub fn is_string_to_value_map(&self) -> bool {
        matches!(self, Self::StringToValueMap(_))
    }

    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_ten_confirmation_id(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTenConfirmationId(_))
    }

    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_ten_affected_users(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTenAffectedUsers(_))
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

    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_content_ten_confirmation_id(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTenConfirmationId> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTenConfirmationId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_content_ten_confirmation_id(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTenConfirmationId> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTenConfirmationId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_content_ten_affected_users(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTenAffectedUsers> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTenAffectedUsers(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_content_ten_affected_users(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTenAffectedUsers> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTenAffectedUsers(value) => Some(value),
                    _ => None,
                }
    }
}
