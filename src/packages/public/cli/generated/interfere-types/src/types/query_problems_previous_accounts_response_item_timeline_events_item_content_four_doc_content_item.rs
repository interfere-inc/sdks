pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocContentItem {
        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocContentItemZero(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocContentItemZero),

        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocContentItemOne(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocContentItemOne),

        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocContentItemType(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocContentItemType),
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocContentItem {
    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_four_doc_content_item_zero(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocContentItemZero(_))
    }

    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_four_doc_content_item_one(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocContentItemOne(_))
    }

    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_four_doc_content_item_type(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocContentItemType(_))
    }


    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_content_four_doc_content_item_zero(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocContentItemZero> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocContentItemZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_content_four_doc_content_item_zero(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocContentItemZero> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocContentItemZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_content_four_doc_content_item_one(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocContentItemOne> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocContentItemOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_content_four_doc_content_item_one(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocContentItemOne> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocContentItemOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_content_four_doc_content_item_type(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocContentItemType> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocContentItemType(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_content_four_doc_content_item_type(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocContentItemType> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocContentItemType(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocContentItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocContentItemZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocContentItemOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocContentItemType(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
