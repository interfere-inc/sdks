pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum TimelineEventRequestContractRequiredEvidenceKind {
        TimelineEventRequestContractRequiredEvidenceKindZero(TimelineEventRequestContractRequiredEvidenceKindZero),

        TimelineEventRequestContractRequiredEvidenceKindOne(TimelineEventRequestContractRequiredEvidenceKindOne),
}

impl TimelineEventRequestContractRequiredEvidenceKind {
    pub fn is_timeline_event_request_contract_required_evidence_kind_zero(&self) -> bool {
        matches!(self, Self::TimelineEventRequestContractRequiredEvidenceKindZero(_))
    }

    pub fn is_timeline_event_request_contract_required_evidence_kind_one(&self) -> bool {
        matches!(self, Self::TimelineEventRequestContractRequiredEvidenceKindOne(_))
    }


    pub fn as_timeline_event_request_contract_required_evidence_kind_zero(&self) -> Option<&TimelineEventRequestContractRequiredEvidenceKindZero> {
        match self {
                    Self::TimelineEventRequestContractRequiredEvidenceKindZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_request_contract_required_evidence_kind_zero(self) -> Option<TimelineEventRequestContractRequiredEvidenceKindZero> {
        match self {
                    Self::TimelineEventRequestContractRequiredEvidenceKindZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_request_contract_required_evidence_kind_one(&self) -> Option<&TimelineEventRequestContractRequiredEvidenceKindOne> {
        match self {
                    Self::TimelineEventRequestContractRequiredEvidenceKindOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_request_contract_required_evidence_kind_one(self) -> Option<TimelineEventRequestContractRequiredEvidenceKindOne> {
        match self {
                    Self::TimelineEventRequestContractRequiredEvidenceKindOne(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for TimelineEventRequestContractRequiredEvidenceKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TimelineEventRequestContractRequiredEvidenceKindZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventRequestContractRequiredEvidenceKindOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
