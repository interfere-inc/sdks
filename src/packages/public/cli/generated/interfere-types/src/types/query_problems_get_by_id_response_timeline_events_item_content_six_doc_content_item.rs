pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItem {
        QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemZero(QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemZero),

        QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemOne(QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemOne),

        QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemType(QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemType),
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItem {
    pub fn is_query_problems_get_by_id_response_timeline_events_item_content_six_doc_content_item_zero(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemZero(_))
    }

    pub fn is_query_problems_get_by_id_response_timeline_events_item_content_six_doc_content_item_one(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemOne(_))
    }

    pub fn is_query_problems_get_by_id_response_timeline_events_item_content_six_doc_content_item_type(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemType(_))
    }


    pub fn as_query_problems_get_by_id_response_timeline_events_item_content_six_doc_content_item_zero(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemZero> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_content_six_doc_content_item_zero(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemZero> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_get_by_id_response_timeline_events_item_content_six_doc_content_item_one(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemOne> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_content_six_doc_content_item_one(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemOne> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_get_by_id_response_timeline_events_item_content_six_doc_content_item_type(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemType> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemType(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_content_six_doc_content_item_type(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemType> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemType(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemType(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
