pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItem {
        QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemZero(QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemZero),

        QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemOne(QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemOne),

        QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemType(QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemType),
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItem {
    pub fn is_query_problems_get_by_id_response_timeline_events_item_content_four_doc_content_item_zero(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemZero(_))
    }

    pub fn is_query_problems_get_by_id_response_timeline_events_item_content_four_doc_content_item_one(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemOne(_))
    }

    pub fn is_query_problems_get_by_id_response_timeline_events_item_content_four_doc_content_item_type(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemType(_))
    }


    pub fn as_query_problems_get_by_id_response_timeline_events_item_content_four_doc_content_item_zero(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemZero> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_content_four_doc_content_item_zero(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemZero> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_get_by_id_response_timeline_events_item_content_four_doc_content_item_one(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemOne> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_content_four_doc_content_item_one(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemOne> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_get_by_id_response_timeline_events_item_content_four_doc_content_item_type(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemType> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemType(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_content_four_doc_content_item_type(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemType> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemType(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemType(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
