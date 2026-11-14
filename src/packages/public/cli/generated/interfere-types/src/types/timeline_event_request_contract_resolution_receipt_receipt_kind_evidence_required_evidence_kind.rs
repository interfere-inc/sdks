pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum TimelineEventRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceKind {
        TimelineEventRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceKindZero(TimelineEventRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceKindZero),

        TimelineEventRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceKindOne(TimelineEventRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceKindOne),
}

impl TimelineEventRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceKind {
    pub fn is_timeline_event_request_contract_resolution_receipt_receipt_kind_evidence_required_evidence_kind_zero(&self) -> bool {
        matches!(self, Self::TimelineEventRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceKindZero(_))
    }

    pub fn is_timeline_event_request_contract_resolution_receipt_receipt_kind_evidence_required_evidence_kind_one(&self) -> bool {
        matches!(self, Self::TimelineEventRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceKindOne(_))
    }


    pub fn as_timeline_event_request_contract_resolution_receipt_receipt_kind_evidence_required_evidence_kind_zero(&self) -> Option<&TimelineEventRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceKindZero> {
        match self {
                    Self::TimelineEventRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceKindZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_request_contract_resolution_receipt_receipt_kind_evidence_required_evidence_kind_zero(self) -> Option<TimelineEventRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceKindZero> {
        match self {
                    Self::TimelineEventRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceKindZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_request_contract_resolution_receipt_receipt_kind_evidence_required_evidence_kind_one(&self) -> Option<&TimelineEventRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceKindOne> {
        match self {
                    Self::TimelineEventRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceKindOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_request_contract_resolution_receipt_receipt_kind_evidence_required_evidence_kind_one(self) -> Option<TimelineEventRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceKindOne> {
        match self {
                    Self::TimelineEventRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceKindOne(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for TimelineEventRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TimelineEventRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceKindZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceKindOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
