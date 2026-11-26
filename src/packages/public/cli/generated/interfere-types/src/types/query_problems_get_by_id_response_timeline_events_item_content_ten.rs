pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum QueryProblemsGetByIdResponseTimelineEventsItemContentTen {
        StringToValueMap(HashMap<String, serde_json::Value>),

        QueryProblemsGetByIdResponseTimelineEventsItemContentTenConfirmationId(QueryProblemsGetByIdResponseTimelineEventsItemContentTenConfirmationId),

        QueryProblemsGetByIdResponseTimelineEventsItemContentTenAffectedUsers(QueryProblemsGetByIdResponseTimelineEventsItemContentTenAffectedUsers),
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentTen {
    pub fn is_string_to_value_map(&self) -> bool {
        matches!(self, Self::StringToValueMap(_))
    }

    pub fn is_query_problems_get_by_id_response_timeline_events_item_content_ten_confirmation_id(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemContentTenConfirmationId(_))
    }

    pub fn is_query_problems_get_by_id_response_timeline_events_item_content_ten_affected_users(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemContentTenAffectedUsers(_))
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

    pub fn as_query_problems_get_by_id_response_timeline_events_item_content_ten_confirmation_id(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemContentTenConfirmationId> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentTenConfirmationId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_content_ten_confirmation_id(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemContentTenConfirmationId> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentTenConfirmationId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_get_by_id_response_timeline_events_item_content_ten_affected_users(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemContentTenAffectedUsers> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentTenAffectedUsers(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_content_ten_affected_users(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemContentTenAffectedUsers> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentTenAffectedUsers(value) => Some(value),
                    _ => None,
                }
    }
}
