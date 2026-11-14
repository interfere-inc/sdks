pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum TimelineEventContentActionKindRequiredEvidenceKind {
        TimelineEventContentActionKindRequiredEvidenceKindZero(TimelineEventContentActionKindRequiredEvidenceKindZero),

        TimelineEventContentActionKindRequiredEvidenceKindOne(TimelineEventContentActionKindRequiredEvidenceKindOne),
}

impl TimelineEventContentActionKindRequiredEvidenceKind {
    pub fn is_timeline_event_content_action_kind_required_evidence_kind_zero(&self) -> bool {
        matches!(self, Self::TimelineEventContentActionKindRequiredEvidenceKindZero(_))
    }

    pub fn is_timeline_event_content_action_kind_required_evidence_kind_one(&self) -> bool {
        matches!(self, Self::TimelineEventContentActionKindRequiredEvidenceKindOne(_))
    }


    pub fn as_timeline_event_content_action_kind_required_evidence_kind_zero(&self) -> Option<&TimelineEventContentActionKindRequiredEvidenceKindZero> {
        match self {
                    Self::TimelineEventContentActionKindRequiredEvidenceKindZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_action_kind_required_evidence_kind_zero(self) -> Option<TimelineEventContentActionKindRequiredEvidenceKindZero> {
        match self {
                    Self::TimelineEventContentActionKindRequiredEvidenceKindZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_action_kind_required_evidence_kind_one(&self) -> Option<&TimelineEventContentActionKindRequiredEvidenceKindOne> {
        match self {
                    Self::TimelineEventContentActionKindRequiredEvidenceKindOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_action_kind_required_evidence_kind_one(self) -> Option<TimelineEventContentActionKindRequiredEvidenceKindOne> {
        match self {
                    Self::TimelineEventContentActionKindRequiredEvidenceKindOne(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for TimelineEventContentActionKindRequiredEvidenceKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TimelineEventContentActionKindRequiredEvidenceKindZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentActionKindRequiredEvidenceKindOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
