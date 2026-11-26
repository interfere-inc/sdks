pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelve {
        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCause(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCause),

        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveOne(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveOne),
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelve {
    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_twelve_cause(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCause(_))
    }

    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_twelve_one(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveOne(_))
    }


    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_content_twelve_cause(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCause> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCause(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_content_twelve_cause(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCause> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCause(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_content_twelve_one(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveOne> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_content_twelve_one(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveOne> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveOne(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelve {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCause(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
