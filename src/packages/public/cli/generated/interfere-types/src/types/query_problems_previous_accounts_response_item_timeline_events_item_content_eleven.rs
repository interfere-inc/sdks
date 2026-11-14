pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEleven {
        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenCause(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenCause),

        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOne(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOne),
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEleven {
    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_eleven_cause(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenCause(_))
    }

    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_eleven_one(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOne(_))
    }


    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_content_eleven_cause(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenCause> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenCause(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_content_eleven_cause(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenCause> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenCause(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_content_eleven_one(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOne> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_content_eleven_one(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOne> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOne(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEleven {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenCause(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
