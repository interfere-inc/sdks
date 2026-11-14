pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceKind {
        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceKindZero(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceKindZero),

        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceKindOne(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceKindOne),
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceKind {
    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_action_kind_evidence_status_receipt_required_evidence_kind_zero(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceKindZero(_))
    }

    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_action_kind_evidence_status_receipt_required_evidence_kind_one(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceKindOne(_))
    }


    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_content_action_kind_evidence_status_receipt_required_evidence_kind_zero(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceKindZero> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceKindZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_content_action_kind_evidence_status_receipt_required_evidence_kind_zero(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceKindZero> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceKindZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_content_action_kind_evidence_status_receipt_required_evidence_kind_one(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceKindOne> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceKindOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_content_action_kind_evidence_status_receipt_required_evidence_kind_one(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceKindOne> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceKindOne(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceKindZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceKindOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
