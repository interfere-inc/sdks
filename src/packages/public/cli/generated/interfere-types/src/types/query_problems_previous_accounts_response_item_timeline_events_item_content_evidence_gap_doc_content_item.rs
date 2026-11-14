pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItem {
        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemZero(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemZero),

        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemOne(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemOne),

        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemType(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemType),
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItem {
    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_evidence_gap_doc_content_item_zero(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemZero(_))
    }

    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_evidence_gap_doc_content_item_one(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemOne(_))
    }

    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_evidence_gap_doc_content_item_type(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemType(_))
    }


    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_content_evidence_gap_doc_content_item_zero(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemZero> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_content_evidence_gap_doc_content_item_zero(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemZero> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_content_evidence_gap_doc_content_item_one(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemOne> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_content_evidence_gap_doc_content_item_one(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemOne> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_content_evidence_gap_doc_content_item_type(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemType> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemType(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_content_evidence_gap_doc_content_item_type(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemType> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemType(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemType(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
