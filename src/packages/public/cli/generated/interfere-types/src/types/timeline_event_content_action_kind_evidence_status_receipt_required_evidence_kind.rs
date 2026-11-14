pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidenceKind {
        TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidenceKindZero(TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidenceKindZero),

        TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidenceKindOne(TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidenceKindOne),
}

impl TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidenceKind {
    pub fn is_timeline_event_content_action_kind_evidence_status_receipt_required_evidence_kind_zero(&self) -> bool {
        matches!(self, Self::TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidenceKindZero(_))
    }

    pub fn is_timeline_event_content_action_kind_evidence_status_receipt_required_evidence_kind_one(&self) -> bool {
        matches!(self, Self::TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidenceKindOne(_))
    }


    pub fn as_timeline_event_content_action_kind_evidence_status_receipt_required_evidence_kind_zero(&self) -> Option<&TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidenceKindZero> {
        match self {
                    Self::TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidenceKindZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_action_kind_evidence_status_receipt_required_evidence_kind_zero(self) -> Option<TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidenceKindZero> {
        match self {
                    Self::TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidenceKindZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_action_kind_evidence_status_receipt_required_evidence_kind_one(&self) -> Option<&TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidenceKindOne> {
        match self {
                    Self::TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidenceKindOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_action_kind_evidence_status_receipt_required_evidence_kind_one(self) -> Option<TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidenceKindOne> {
        match self {
                    Self::TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidenceKindOne(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidenceKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidenceKindZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidenceKindOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
