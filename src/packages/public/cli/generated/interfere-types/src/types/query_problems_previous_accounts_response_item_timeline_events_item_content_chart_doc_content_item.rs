pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocContentItem {
        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocContentItemZero(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocContentItemZero),

        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocContentItemOne(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocContentItemOne),

        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocContentItemType(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocContentItemType),
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocContentItem {
    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_chart_doc_content_item_zero(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocContentItemZero(_))
    }

    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_chart_doc_content_item_one(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocContentItemOne(_))
    }

    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_chart_doc_content_item_type(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocContentItemType(_))
    }


    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_content_chart_doc_content_item_zero(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocContentItemZero> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocContentItemZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_content_chart_doc_content_item_zero(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocContentItemZero> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocContentItemZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_content_chart_doc_content_item_one(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocContentItemOne> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocContentItemOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_content_chart_doc_content_item_one(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocContentItemOne> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocContentItemOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_content_chart_doc_content_item_type(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocContentItemType> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocContentItemType(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_content_chart_doc_content_item_type(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocContentItemType> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocContentItemType(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocContentItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocContentItemZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocContentItemOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocContentItemType(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
