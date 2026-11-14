pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum QueryProblemsGetByIdResponseTimelineEventsItemContentEleven {
        QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCause(QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCause),

        QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOne(QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOne),
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentEleven {
    pub fn is_query_problems_get_by_id_response_timeline_events_item_content_eleven_cause(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCause(_))
    }

    pub fn is_query_problems_get_by_id_response_timeline_events_item_content_eleven_one(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOne(_))
    }


    pub fn as_query_problems_get_by_id_response_timeline_events_item_content_eleven_cause(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCause> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCause(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_content_eleven_cause(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCause> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCause(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_get_by_id_response_timeline_events_item_content_eleven_one(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOne> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_content_eleven_one(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOne> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOne(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryProblemsGetByIdResponseTimelineEventsItemContentEleven {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCause(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
