pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceKind {
        QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceKindZero(QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceKindZero),

        QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceKindOne(QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceKindOne),
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceKind {
    pub fn is_query_problems_get_by_id_response_timeline_events_item_content_action_kind_evidence_status_receipt_required_evidence_kind_zero(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceKindZero(_))
    }

    pub fn is_query_problems_get_by_id_response_timeline_events_item_content_action_kind_evidence_status_receipt_required_evidence_kind_one(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceKindOne(_))
    }


    pub fn as_query_problems_get_by_id_response_timeline_events_item_content_action_kind_evidence_status_receipt_required_evidence_kind_zero(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceKindZero> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceKindZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_content_action_kind_evidence_status_receipt_required_evidence_kind_zero(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceKindZero> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceKindZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_get_by_id_response_timeline_events_item_content_action_kind_evidence_status_receipt_required_evidence_kind_one(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceKindOne> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceKindOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_content_action_kind_evidence_status_receipt_required_evidence_kind_one(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceKindOne> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceKindOne(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceKindZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceKindOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
