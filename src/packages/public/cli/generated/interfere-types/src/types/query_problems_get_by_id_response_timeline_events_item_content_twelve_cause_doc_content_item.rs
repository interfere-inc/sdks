pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseDocContentItem {
        QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseDocContentItemZero(QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseDocContentItemZero),

        QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseDocContentItemOne(QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseDocContentItemOne),

        QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseDocContentItemType(QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseDocContentItemType),
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseDocContentItem {
    pub fn is_query_problems_get_by_id_response_timeline_events_item_content_twelve_cause_doc_content_item_zero(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseDocContentItemZero(_))
    }

    pub fn is_query_problems_get_by_id_response_timeline_events_item_content_twelve_cause_doc_content_item_one(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseDocContentItemOne(_))
    }

    pub fn is_query_problems_get_by_id_response_timeline_events_item_content_twelve_cause_doc_content_item_type(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseDocContentItemType(_))
    }


    pub fn as_query_problems_get_by_id_response_timeline_events_item_content_twelve_cause_doc_content_item_zero(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseDocContentItemZero> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseDocContentItemZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_content_twelve_cause_doc_content_item_zero(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseDocContentItemZero> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseDocContentItemZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_get_by_id_response_timeline_events_item_content_twelve_cause_doc_content_item_one(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseDocContentItemOne> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseDocContentItemOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_content_twelve_cause_doc_content_item_one(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseDocContentItemOne> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseDocContentItemOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_get_by_id_response_timeline_events_item_content_twelve_cause_doc_content_item_type(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseDocContentItemType> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseDocContentItemType(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_content_twelve_cause_doc_content_item_type(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseDocContentItemType> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseDocContentItemType(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseDocContentItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseDocContentItemZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseDocContentItemOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseDocContentItemType(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
