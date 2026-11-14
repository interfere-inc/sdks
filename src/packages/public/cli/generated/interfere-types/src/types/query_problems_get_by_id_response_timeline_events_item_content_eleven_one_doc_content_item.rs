pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItem {
        QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemZero(QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemZero),

        QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemOne(QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemOne),

        QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemType(QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemType),
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItem {
    pub fn is_query_problems_get_by_id_response_timeline_events_item_content_eleven_one_doc_content_item_zero(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemZero(_))
    }

    pub fn is_query_problems_get_by_id_response_timeline_events_item_content_eleven_one_doc_content_item_one(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemOne(_))
    }

    pub fn is_query_problems_get_by_id_response_timeline_events_item_content_eleven_one_doc_content_item_type(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemType(_))
    }


    pub fn as_query_problems_get_by_id_response_timeline_events_item_content_eleven_one_doc_content_item_zero(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemZero> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_content_eleven_one_doc_content_item_zero(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemZero> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_get_by_id_response_timeline_events_item_content_eleven_one_doc_content_item_one(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemOne> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_content_eleven_one_doc_content_item_one(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemOne> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_get_by_id_response_timeline_events_item_content_eleven_one_doc_content_item_type(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemType> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemType(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_content_eleven_one_doc_content_item_type(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemType> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemType(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemType(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
