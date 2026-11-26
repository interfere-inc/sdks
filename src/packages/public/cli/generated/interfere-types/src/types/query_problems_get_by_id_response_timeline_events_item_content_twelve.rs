pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum QueryProblemsGetByIdResponseTimelineEventsItemContentTwelve {
        QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCause(QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCause),

        QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOne(QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOne),
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentTwelve {
    pub fn is_query_problems_get_by_id_response_timeline_events_item_content_twelve_cause(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCause(_))
    }

    pub fn is_query_problems_get_by_id_response_timeline_events_item_content_twelve_one(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOne(_))
    }


    pub fn as_query_problems_get_by_id_response_timeline_events_item_content_twelve_cause(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCause> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCause(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_content_twelve_cause(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCause> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCause(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_get_by_id_response_timeline_events_item_content_twelve_one(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOne> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_content_twelve_one(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOne> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOne(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryProblemsGetByIdResponseTimelineEventsItemContentTwelve {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCause(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
