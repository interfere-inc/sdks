pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum TimelineEventContentEvidenceGapPublication {
        TimelineEventContentEvidenceGapPublicationBasis(TimelineEventContentEvidenceGapPublicationBasis),

        TimelineEventContentEvidenceGapPublicationReason(TimelineEventContentEvidenceGapPublicationReason),
}

impl TimelineEventContentEvidenceGapPublication {
    pub fn is_timeline_event_content_evidence_gap_publication_basis(&self) -> bool {
        matches!(self, Self::TimelineEventContentEvidenceGapPublicationBasis(_))
    }

    pub fn is_timeline_event_content_evidence_gap_publication_reason(&self) -> bool {
        matches!(self, Self::TimelineEventContentEvidenceGapPublicationReason(_))
    }


    pub fn as_timeline_event_content_evidence_gap_publication_basis(&self) -> Option<&TimelineEventContentEvidenceGapPublicationBasis> {
        match self {
                    Self::TimelineEventContentEvidenceGapPublicationBasis(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_evidence_gap_publication_basis(self) -> Option<TimelineEventContentEvidenceGapPublicationBasis> {
        match self {
                    Self::TimelineEventContentEvidenceGapPublicationBasis(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_evidence_gap_publication_reason(&self) -> Option<&TimelineEventContentEvidenceGapPublicationReason> {
        match self {
                    Self::TimelineEventContentEvidenceGapPublicationReason(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_evidence_gap_publication_reason(self) -> Option<TimelineEventContentEvidenceGapPublicationReason> {
        match self {
                    Self::TimelineEventContentEvidenceGapPublicationReason(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for TimelineEventContentEvidenceGapPublication {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TimelineEventContentEvidenceGapPublicationBasis(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentEvidenceGapPublicationReason(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
