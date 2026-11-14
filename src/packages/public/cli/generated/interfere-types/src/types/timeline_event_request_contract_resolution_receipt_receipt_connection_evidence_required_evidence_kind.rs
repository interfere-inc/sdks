pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceKind {
        TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceKindZero(TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceKindZero),

        TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceKindOne(TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceKindOne),
}

impl TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceKind {
    pub fn is_timeline_event_request_contract_resolution_receipt_receipt_connection_evidence_required_evidence_kind_zero(&self) -> bool {
        matches!(self, Self::TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceKindZero(_))
    }

    pub fn is_timeline_event_request_contract_resolution_receipt_receipt_connection_evidence_required_evidence_kind_one(&self) -> bool {
        matches!(self, Self::TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceKindOne(_))
    }


    pub fn as_timeline_event_request_contract_resolution_receipt_receipt_connection_evidence_required_evidence_kind_zero(&self) -> Option<&TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceKindZero> {
        match self {
                    Self::TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceKindZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_request_contract_resolution_receipt_receipt_connection_evidence_required_evidence_kind_zero(self) -> Option<TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceKindZero> {
        match self {
                    Self::TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceKindZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_request_contract_resolution_receipt_receipt_connection_evidence_required_evidence_kind_one(&self) -> Option<&TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceKindOne> {
        match self {
                    Self::TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceKindOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_request_contract_resolution_receipt_receipt_connection_evidence_required_evidence_kind_one(self) -> Option<TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceKindOne> {
        match self {
                    Self::TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceKindOne(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceKindZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceKindOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
