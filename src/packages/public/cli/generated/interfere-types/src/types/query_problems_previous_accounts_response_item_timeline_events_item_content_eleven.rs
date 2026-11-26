pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEleven {
        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenFixReleaseId(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenFixReleaseId),

        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenCosineScore(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenCosineScore),
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEleven {
    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_eleven_fix_release_id(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenFixReleaseId(_))
    }

    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_eleven_cosine_score(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenCosineScore(_))
    }


    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_content_eleven_fix_release_id(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenFixReleaseId> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenFixReleaseId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_content_eleven_fix_release_id(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenFixReleaseId> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenFixReleaseId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_content_eleven_cosine_score(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenCosineScore> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenCosineScore(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_content_eleven_cosine_score(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenCosineScore> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenCosineScore(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEleven {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenFixReleaseId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenCosineScore(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
