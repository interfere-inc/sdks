pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOneDocContentItem {
        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOneDocContentItemZero(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOneDocContentItemZero),

        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOneDocContentItemOne(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOneDocContentItemOne),

        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOneDocContentItemType(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOneDocContentItemType),
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOneDocContentItem {
    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_eleven_one_doc_content_item_zero(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOneDocContentItemZero(_))
    }

    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_eleven_one_doc_content_item_one(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOneDocContentItemOne(_))
    }

    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_eleven_one_doc_content_item_type(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOneDocContentItemType(_))
    }


    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_content_eleven_one_doc_content_item_zero(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOneDocContentItemZero> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOneDocContentItemZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_content_eleven_one_doc_content_item_zero(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOneDocContentItemZero> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOneDocContentItemZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_content_eleven_one_doc_content_item_one(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOneDocContentItemOne> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOneDocContentItemOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_content_eleven_one_doc_content_item_one(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOneDocContentItemOne> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOneDocContentItemOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_content_eleven_one_doc_content_item_type(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOneDocContentItemType> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOneDocContentItemType(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_content_eleven_one_doc_content_item_type(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOneDocContentItemType> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOneDocContentItemType(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOneDocContentItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOneDocContentItemZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOneDocContentItemOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenOneDocContentItemType(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
