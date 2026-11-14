pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum QueryProblemsGetByIdResponseTimelineEventsItemContentNine {
        StringToValueMap(HashMap<String, serde_json::Value>),

        QueryProblemsGetByIdResponseTimelineEventsItemContentNineConfirmationId(QueryProblemsGetByIdResponseTimelineEventsItemContentNineConfirmationId),

        QueryProblemsGetByIdResponseTimelineEventsItemContentNineAffectedUsers(QueryProblemsGetByIdResponseTimelineEventsItemContentNineAffectedUsers),
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentNine {
    pub fn is_string_to_value_map(&self) -> bool {
        matches!(self, Self::StringToValueMap(_))
    }

    pub fn is_query_problems_get_by_id_response_timeline_events_item_content_nine_confirmation_id(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemContentNineConfirmationId(_))
    }

    pub fn is_query_problems_get_by_id_response_timeline_events_item_content_nine_affected_users(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemContentNineAffectedUsers(_))
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

    pub fn as_query_problems_get_by_id_response_timeline_events_item_content_nine_confirmation_id(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemContentNineConfirmationId> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentNineConfirmationId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_content_nine_confirmation_id(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemContentNineConfirmationId> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentNineConfirmationId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_get_by_id_response_timeline_events_item_content_nine_affected_users(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemContentNineAffectedUsers> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentNineAffectedUsers(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_content_nine_affected_users(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemContentNineAffectedUsers> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentNineAffectedUsers(value) => Some(value),
                    _ => None,
                }
    }
}
