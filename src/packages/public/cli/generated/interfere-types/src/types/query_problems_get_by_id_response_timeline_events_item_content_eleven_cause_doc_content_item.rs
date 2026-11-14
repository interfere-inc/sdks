pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocContentItem {
        QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocContentItemZero(QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocContentItemZero),

        QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocContentItemOne(QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocContentItemOne),

        QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocContentItemType(QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocContentItemType),
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocContentItem {
    pub fn is_query_problems_get_by_id_response_timeline_events_item_content_eleven_cause_doc_content_item_zero(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocContentItemZero(_))
    }

    pub fn is_query_problems_get_by_id_response_timeline_events_item_content_eleven_cause_doc_content_item_one(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocContentItemOne(_))
    }

    pub fn is_query_problems_get_by_id_response_timeline_events_item_content_eleven_cause_doc_content_item_type(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocContentItemType(_))
    }


    pub fn as_query_problems_get_by_id_response_timeline_events_item_content_eleven_cause_doc_content_item_zero(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocContentItemZero> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocContentItemZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_content_eleven_cause_doc_content_item_zero(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocContentItemZero> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocContentItemZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_get_by_id_response_timeline_events_item_content_eleven_cause_doc_content_item_one(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocContentItemOne> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocContentItemOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_content_eleven_cause_doc_content_item_one(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocContentItemOne> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocContentItemOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_get_by_id_response_timeline_events_item_content_eleven_cause_doc_content_item_type(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocContentItemType> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocContentItemType(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_content_eleven_cause_doc_content_item_type(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocContentItemType> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocContentItemType(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocContentItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocContentItemZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocContentItemOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocContentItemType(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
