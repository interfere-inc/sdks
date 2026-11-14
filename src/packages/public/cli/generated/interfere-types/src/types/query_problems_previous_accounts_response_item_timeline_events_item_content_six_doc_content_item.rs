pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocContentItem {
        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocContentItemZero(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocContentItemZero),

        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocContentItemOne(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocContentItemOne),

        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocContentItemType(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocContentItemType),
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocContentItem {
    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_six_doc_content_item_zero(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocContentItemZero(_))
    }

    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_six_doc_content_item_one(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocContentItemOne(_))
    }

    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_six_doc_content_item_type(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocContentItemType(_))
    }


    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_content_six_doc_content_item_zero(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocContentItemZero> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocContentItemZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_content_six_doc_content_item_zero(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocContentItemZero> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocContentItemZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_content_six_doc_content_item_one(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocContentItemOne> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocContentItemOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_content_six_doc_content_item_one(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocContentItemOne> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocContentItemOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_content_six_doc_content_item_type(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocContentItemType> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocContentItemType(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_content_six_doc_content_item_type(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocContentItemType> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocContentItemType(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocContentItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocContentItemZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocContentItemOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocContentItemType(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
