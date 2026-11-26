pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOneDocContentItem {
        QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOneDocContentItemZero(QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOneDocContentItemZero),

        QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOneDocContentItemOne(QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOneDocContentItemOne),

        QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOneDocContentItemType(QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOneDocContentItemType),
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOneDocContentItem {
    pub fn is_query_problems_get_by_id_response_timeline_events_item_content_twelve_one_doc_content_item_zero(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOneDocContentItemZero(_))
    }

    pub fn is_query_problems_get_by_id_response_timeline_events_item_content_twelve_one_doc_content_item_one(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOneDocContentItemOne(_))
    }

    pub fn is_query_problems_get_by_id_response_timeline_events_item_content_twelve_one_doc_content_item_type(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOneDocContentItemType(_))
    }


    pub fn as_query_problems_get_by_id_response_timeline_events_item_content_twelve_one_doc_content_item_zero(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOneDocContentItemZero> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOneDocContentItemZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_content_twelve_one_doc_content_item_zero(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOneDocContentItemZero> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOneDocContentItemZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_get_by_id_response_timeline_events_item_content_twelve_one_doc_content_item_one(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOneDocContentItemOne> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOneDocContentItemOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_content_twelve_one_doc_content_item_one(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOneDocContentItemOne> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOneDocContentItemOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_get_by_id_response_timeline_events_item_content_twelve_one_doc_content_item_type(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOneDocContentItemType> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOneDocContentItemType(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_content_twelve_one_doc_content_item_type(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOneDocContentItemType> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOneDocContentItemType(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOneDocContentItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOneDocContentItemZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOneDocContentItemOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOneDocContentItemType(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
