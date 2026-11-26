pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum QueryProblemsGetByIdResponseTimelineEventsItemContentEleven {
        QueryProblemsGetByIdResponseTimelineEventsItemContentElevenFixReleaseId(QueryProblemsGetByIdResponseTimelineEventsItemContentElevenFixReleaseId),

        QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCosineScore(QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCosineScore),
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentEleven {
    pub fn is_query_problems_get_by_id_response_timeline_events_item_content_eleven_fix_release_id(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemContentElevenFixReleaseId(_))
    }

    pub fn is_query_problems_get_by_id_response_timeline_events_item_content_eleven_cosine_score(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCosineScore(_))
    }


    pub fn as_query_problems_get_by_id_response_timeline_events_item_content_eleven_fix_release_id(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemContentElevenFixReleaseId> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentElevenFixReleaseId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_content_eleven_fix_release_id(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemContentElevenFixReleaseId> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentElevenFixReleaseId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_get_by_id_response_timeline_events_item_content_eleven_cosine_score(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCosineScore> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCosineScore(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_content_eleven_cosine_score(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCosineScore> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCosineScore(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryProblemsGetByIdResponseTimelineEventsItemContentEleven {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryProblemsGetByIdResponseTimelineEventsItemContentElevenFixReleaseId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCosineScore(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
