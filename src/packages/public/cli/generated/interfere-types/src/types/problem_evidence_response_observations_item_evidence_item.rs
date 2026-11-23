pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ProblemEvidenceResponseObservationsItemEvidenceItem {
        ProblemEvidenceResponseObservationsItemEvidenceItemSourceFrame(ProblemEvidenceResponseObservationsItemEvidenceItemSourceFrame),

        ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevision(ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevision),

        ProblemEvidenceResponseObservationsItemEvidenceItemId(ProblemEvidenceResponseObservationsItemEvidenceItemId),

        ProblemEvidenceResponseObservationsItemEvidenceItemEndLine(ProblemEvidenceResponseObservationsItemEvidenceItemEndLine),

        ProblemEvidenceResponseObservationsItemEvidenceItemReleaseId(ProblemEvidenceResponseObservationsItemEvidenceItemReleaseId),

        ProblemEvidenceResponseObservationsItemEvidenceItemFive(ProblemEvidenceResponseObservationsItemEvidenceItemFive),

        ProblemEvidenceResponseObservationsItemEvidenceItemSix(ProblemEvidenceResponseObservationsItemEvidenceItemSix),

        ProblemEvidenceResponseObservationsItemEvidenceItemAffectedUsers(ProblemEvidenceResponseObservationsItemEvidenceItemAffectedUsers),

        ProblemEvidenceResponseObservationsItemEvidenceItemAfterEventCount(ProblemEvidenceResponseObservationsItemEvidenceItemAfterEventCount),

        ProblemEvidenceResponseObservationsItemEvidenceItemRecordCount(ProblemEvidenceResponseObservationsItemEvidenceItemRecordCount),

        ProblemEvidenceResponseObservationsItemEvidenceItemEvidenceDigest(ProblemEvidenceResponseObservationsItemEvidenceItemEvidenceDigest),

        ProblemEvidenceResponseObservationsItemEvidenceItemMinSeverity(ProblemEvidenceResponseObservationsItemEvidenceItemMinSeverity),

        ProblemEvidenceResponseObservationsItemEvidenceItemContentDigest(ProblemEvidenceResponseObservationsItemEvidenceItemContentDigest),
}

impl ProblemEvidenceResponseObservationsItemEvidenceItem {
    pub fn is_problem_evidence_response_observations_item_evidence_item_source_frame(&self) -> bool {
        matches!(self, Self::ProblemEvidenceResponseObservationsItemEvidenceItemSourceFrame(_))
    }

    pub fn is_problem_evidence_response_observations_item_evidence_item_connection_revision(&self) -> bool {
        matches!(self, Self::ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevision(_))
    }

    pub fn is_problem_evidence_response_observations_item_evidence_item_id(&self) -> bool {
        matches!(self, Self::ProblemEvidenceResponseObservationsItemEvidenceItemId(_))
    }

    pub fn is_problem_evidence_response_observations_item_evidence_item_end_line(&self) -> bool {
        matches!(self, Self::ProblemEvidenceResponseObservationsItemEvidenceItemEndLine(_))
    }

    pub fn is_problem_evidence_response_observations_item_evidence_item_release_id(&self) -> bool {
        matches!(self, Self::ProblemEvidenceResponseObservationsItemEvidenceItemReleaseId(_))
    }

    pub fn is_problem_evidence_response_observations_item_evidence_item_five(&self) -> bool {
        matches!(self, Self::ProblemEvidenceResponseObservationsItemEvidenceItemFive(_))
    }

    pub fn is_problem_evidence_response_observations_item_evidence_item_six(&self) -> bool {
        matches!(self, Self::ProblemEvidenceResponseObservationsItemEvidenceItemSix(_))
    }

    pub fn is_problem_evidence_response_observations_item_evidence_item_affected_users(&self) -> bool {
        matches!(self, Self::ProblemEvidenceResponseObservationsItemEvidenceItemAffectedUsers(_))
    }

    pub fn is_problem_evidence_response_observations_item_evidence_item_after_event_count(&self) -> bool {
        matches!(self, Self::ProblemEvidenceResponseObservationsItemEvidenceItemAfterEventCount(_))
    }

    pub fn is_problem_evidence_response_observations_item_evidence_item_record_count(&self) -> bool {
        matches!(self, Self::ProblemEvidenceResponseObservationsItemEvidenceItemRecordCount(_))
    }

    pub fn is_problem_evidence_response_observations_item_evidence_item_evidence_digest(&self) -> bool {
        matches!(self, Self::ProblemEvidenceResponseObservationsItemEvidenceItemEvidenceDigest(_))
    }

    pub fn is_problem_evidence_response_observations_item_evidence_item_min_severity(&self) -> bool {
        matches!(self, Self::ProblemEvidenceResponseObservationsItemEvidenceItemMinSeverity(_))
    }

    pub fn is_problem_evidence_response_observations_item_evidence_item_content_digest(&self) -> bool {
        matches!(self, Self::ProblemEvidenceResponseObservationsItemEvidenceItemContentDigest(_))
    }


    pub fn as_problem_evidence_response_observations_item_evidence_item_source_frame(&self) -> Option<&ProblemEvidenceResponseObservationsItemEvidenceItemSourceFrame> {
        match self {
                    Self::ProblemEvidenceResponseObservationsItemEvidenceItemSourceFrame(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_problem_evidence_response_observations_item_evidence_item_source_frame(self) -> Option<ProblemEvidenceResponseObservationsItemEvidenceItemSourceFrame> {
        match self {
                    Self::ProblemEvidenceResponseObservationsItemEvidenceItemSourceFrame(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_problem_evidence_response_observations_item_evidence_item_connection_revision(&self) -> Option<&ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevision> {
        match self {
                    Self::ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevision(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_problem_evidence_response_observations_item_evidence_item_connection_revision(self) -> Option<ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevision> {
        match self {
                    Self::ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevision(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_problem_evidence_response_observations_item_evidence_item_id(&self) -> Option<&ProblemEvidenceResponseObservationsItemEvidenceItemId> {
        match self {
                    Self::ProblemEvidenceResponseObservationsItemEvidenceItemId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_problem_evidence_response_observations_item_evidence_item_id(self) -> Option<ProblemEvidenceResponseObservationsItemEvidenceItemId> {
        match self {
                    Self::ProblemEvidenceResponseObservationsItemEvidenceItemId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_problem_evidence_response_observations_item_evidence_item_end_line(&self) -> Option<&ProblemEvidenceResponseObservationsItemEvidenceItemEndLine> {
        match self {
                    Self::ProblemEvidenceResponseObservationsItemEvidenceItemEndLine(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_problem_evidence_response_observations_item_evidence_item_end_line(self) -> Option<ProblemEvidenceResponseObservationsItemEvidenceItemEndLine> {
        match self {
                    Self::ProblemEvidenceResponseObservationsItemEvidenceItemEndLine(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_problem_evidence_response_observations_item_evidence_item_release_id(&self) -> Option<&ProblemEvidenceResponseObservationsItemEvidenceItemReleaseId> {
        match self {
                    Self::ProblemEvidenceResponseObservationsItemEvidenceItemReleaseId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_problem_evidence_response_observations_item_evidence_item_release_id(self) -> Option<ProblemEvidenceResponseObservationsItemEvidenceItemReleaseId> {
        match self {
                    Self::ProblemEvidenceResponseObservationsItemEvidenceItemReleaseId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_problem_evidence_response_observations_item_evidence_item_five(&self) -> Option<&ProblemEvidenceResponseObservationsItemEvidenceItemFive> {
        match self {
                    Self::ProblemEvidenceResponseObservationsItemEvidenceItemFive(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_problem_evidence_response_observations_item_evidence_item_five(self) -> Option<ProblemEvidenceResponseObservationsItemEvidenceItemFive> {
        match self {
                    Self::ProblemEvidenceResponseObservationsItemEvidenceItemFive(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_problem_evidence_response_observations_item_evidence_item_six(&self) -> Option<&ProblemEvidenceResponseObservationsItemEvidenceItemSix> {
        match self {
                    Self::ProblemEvidenceResponseObservationsItemEvidenceItemSix(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_problem_evidence_response_observations_item_evidence_item_six(self) -> Option<ProblemEvidenceResponseObservationsItemEvidenceItemSix> {
        match self {
                    Self::ProblemEvidenceResponseObservationsItemEvidenceItemSix(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_problem_evidence_response_observations_item_evidence_item_affected_users(&self) -> Option<&ProblemEvidenceResponseObservationsItemEvidenceItemAffectedUsers> {
        match self {
                    Self::ProblemEvidenceResponseObservationsItemEvidenceItemAffectedUsers(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_problem_evidence_response_observations_item_evidence_item_affected_users(self) -> Option<ProblemEvidenceResponseObservationsItemEvidenceItemAffectedUsers> {
        match self {
                    Self::ProblemEvidenceResponseObservationsItemEvidenceItemAffectedUsers(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_problem_evidence_response_observations_item_evidence_item_after_event_count(&self) -> Option<&ProblemEvidenceResponseObservationsItemEvidenceItemAfterEventCount> {
        match self {
                    Self::ProblemEvidenceResponseObservationsItemEvidenceItemAfterEventCount(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_problem_evidence_response_observations_item_evidence_item_after_event_count(self) -> Option<ProblemEvidenceResponseObservationsItemEvidenceItemAfterEventCount> {
        match self {
                    Self::ProblemEvidenceResponseObservationsItemEvidenceItemAfterEventCount(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_problem_evidence_response_observations_item_evidence_item_record_count(&self) -> Option<&ProblemEvidenceResponseObservationsItemEvidenceItemRecordCount> {
        match self {
                    Self::ProblemEvidenceResponseObservationsItemEvidenceItemRecordCount(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_problem_evidence_response_observations_item_evidence_item_record_count(self) -> Option<ProblemEvidenceResponseObservationsItemEvidenceItemRecordCount> {
        match self {
                    Self::ProblemEvidenceResponseObservationsItemEvidenceItemRecordCount(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_problem_evidence_response_observations_item_evidence_item_evidence_digest(&self) -> Option<&ProblemEvidenceResponseObservationsItemEvidenceItemEvidenceDigest> {
        match self {
                    Self::ProblemEvidenceResponseObservationsItemEvidenceItemEvidenceDigest(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_problem_evidence_response_observations_item_evidence_item_evidence_digest(self) -> Option<ProblemEvidenceResponseObservationsItemEvidenceItemEvidenceDigest> {
        match self {
                    Self::ProblemEvidenceResponseObservationsItemEvidenceItemEvidenceDigest(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_problem_evidence_response_observations_item_evidence_item_min_severity(&self) -> Option<&ProblemEvidenceResponseObservationsItemEvidenceItemMinSeverity> {
        match self {
                    Self::ProblemEvidenceResponseObservationsItemEvidenceItemMinSeverity(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_problem_evidence_response_observations_item_evidence_item_min_severity(self) -> Option<ProblemEvidenceResponseObservationsItemEvidenceItemMinSeverity> {
        match self {
                    Self::ProblemEvidenceResponseObservationsItemEvidenceItemMinSeverity(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_problem_evidence_response_observations_item_evidence_item_content_digest(&self) -> Option<&ProblemEvidenceResponseObservationsItemEvidenceItemContentDigest> {
        match self {
                    Self::ProblemEvidenceResponseObservationsItemEvidenceItemContentDigest(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_problem_evidence_response_observations_item_evidence_item_content_digest(self) -> Option<ProblemEvidenceResponseObservationsItemEvidenceItemContentDigest> {
        match self {
                    Self::ProblemEvidenceResponseObservationsItemEvidenceItemContentDigest(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for ProblemEvidenceResponseObservationsItemEvidenceItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ProblemEvidenceResponseObservationsItemEvidenceItemSourceFrame(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevision(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ProblemEvidenceResponseObservationsItemEvidenceItemId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ProblemEvidenceResponseObservationsItemEvidenceItemEndLine(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ProblemEvidenceResponseObservationsItemEvidenceItemReleaseId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ProblemEvidenceResponseObservationsItemEvidenceItemFive(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ProblemEvidenceResponseObservationsItemEvidenceItemSix(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ProblemEvidenceResponseObservationsItemEvidenceItemAffectedUsers(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ProblemEvidenceResponseObservationsItemEvidenceItemAfterEventCount(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ProblemEvidenceResponseObservationsItemEvidenceItemRecordCount(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ProblemEvidenceResponseObservationsItemEvidenceItemEvidenceDigest(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ProblemEvidenceResponseObservationsItemEvidenceItemMinSeverity(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ProblemEvidenceResponseObservationsItemEvidenceItemContentDigest(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
