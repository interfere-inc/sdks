pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ProblemEvidenceResponseObservationsItemEvidenceItem {
        ProblemEvidenceResponseObservationsItemEvidenceItemSourceFrame(ProblemEvidenceResponseObservationsItemEvidenceItemSourceFrame),

        ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevision(ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevision),

        ProblemEvidenceResponseObservationsItemEvidenceItemEndLine(ProblemEvidenceResponseObservationsItemEvidenceItemEndLine),

        ProblemEvidenceResponseObservationsItemEvidenceItemReleaseId(ProblemEvidenceResponseObservationsItemEvidenceItemReleaseId),

        ProblemEvidenceResponseObservationsItemEvidenceItemFour(ProblemEvidenceResponseObservationsItemEvidenceItemFour),

        ProblemEvidenceResponseObservationsItemEvidenceItemFive(ProblemEvidenceResponseObservationsItemEvidenceItemFive),

        ProblemEvidenceResponseObservationsItemEvidenceItemAffectedUsers(ProblemEvidenceResponseObservationsItemEvidenceItemAffectedUsers),

        ProblemEvidenceResponseObservationsItemEvidenceItemAfterLimit(ProblemEvidenceResponseObservationsItemEvidenceItemAfterLimit),

        ProblemEvidenceResponseObservationsItemEvidenceItemEight(ProblemEvidenceResponseObservationsItemEvidenceItemEight),

        ProblemEvidenceResponseObservationsItemEvidenceItemEvidenceDigest(ProblemEvidenceResponseObservationsItemEvidenceItemEvidenceDigest),

        ProblemEvidenceResponseObservationsItemEvidenceItemMinSeverity(ProblemEvidenceResponseObservationsItemEvidenceItemMinSeverity),

        ProblemEvidenceResponseObservationsItemEvidenceItemSince(ProblemEvidenceResponseObservationsItemEvidenceItemSince),
}

impl ProblemEvidenceResponseObservationsItemEvidenceItem {
    pub fn is_problem_evidence_response_observations_item_evidence_item_source_frame(&self) -> bool {
        matches!(self, Self::ProblemEvidenceResponseObservationsItemEvidenceItemSourceFrame(_))
    }

    pub fn is_problem_evidence_response_observations_item_evidence_item_connection_revision(&self) -> bool {
        matches!(self, Self::ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevision(_))
    }

    pub fn is_problem_evidence_response_observations_item_evidence_item_end_line(&self) -> bool {
        matches!(self, Self::ProblemEvidenceResponseObservationsItemEvidenceItemEndLine(_))
    }

    pub fn is_problem_evidence_response_observations_item_evidence_item_release_id(&self) -> bool {
        matches!(self, Self::ProblemEvidenceResponseObservationsItemEvidenceItemReleaseId(_))
    }

    pub fn is_problem_evidence_response_observations_item_evidence_item_four(&self) -> bool {
        matches!(self, Self::ProblemEvidenceResponseObservationsItemEvidenceItemFour(_))
    }

    pub fn is_problem_evidence_response_observations_item_evidence_item_five(&self) -> bool {
        matches!(self, Self::ProblemEvidenceResponseObservationsItemEvidenceItemFive(_))
    }

    pub fn is_problem_evidence_response_observations_item_evidence_item_affected_users(&self) -> bool {
        matches!(self, Self::ProblemEvidenceResponseObservationsItemEvidenceItemAffectedUsers(_))
    }

    pub fn is_problem_evidence_response_observations_item_evidence_item_after_limit(&self) -> bool {
        matches!(self, Self::ProblemEvidenceResponseObservationsItemEvidenceItemAfterLimit(_))
    }

    pub fn is_problem_evidence_response_observations_item_evidence_item_eight(&self) -> bool {
        matches!(self, Self::ProblemEvidenceResponseObservationsItemEvidenceItemEight(_))
    }

    pub fn is_problem_evidence_response_observations_item_evidence_item_evidence_digest(&self) -> bool {
        matches!(self, Self::ProblemEvidenceResponseObservationsItemEvidenceItemEvidenceDigest(_))
    }

    pub fn is_problem_evidence_response_observations_item_evidence_item_min_severity(&self) -> bool {
        matches!(self, Self::ProblemEvidenceResponseObservationsItemEvidenceItemMinSeverity(_))
    }

    pub fn is_problem_evidence_response_observations_item_evidence_item_since(&self) -> bool {
        matches!(self, Self::ProblemEvidenceResponseObservationsItemEvidenceItemSince(_))
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

    pub fn as_problem_evidence_response_observations_item_evidence_item_four(&self) -> Option<&ProblemEvidenceResponseObservationsItemEvidenceItemFour> {
        match self {
                    Self::ProblemEvidenceResponseObservationsItemEvidenceItemFour(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_problem_evidence_response_observations_item_evidence_item_four(self) -> Option<ProblemEvidenceResponseObservationsItemEvidenceItemFour> {
        match self {
                    Self::ProblemEvidenceResponseObservationsItemEvidenceItemFour(value) => Some(value),
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

    pub fn as_problem_evidence_response_observations_item_evidence_item_after_limit(&self) -> Option<&ProblemEvidenceResponseObservationsItemEvidenceItemAfterLimit> {
        match self {
                    Self::ProblemEvidenceResponseObservationsItemEvidenceItemAfterLimit(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_problem_evidence_response_observations_item_evidence_item_after_limit(self) -> Option<ProblemEvidenceResponseObservationsItemEvidenceItemAfterLimit> {
        match self {
                    Self::ProblemEvidenceResponseObservationsItemEvidenceItemAfterLimit(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_problem_evidence_response_observations_item_evidence_item_eight(&self) -> Option<&ProblemEvidenceResponseObservationsItemEvidenceItemEight> {
        match self {
                    Self::ProblemEvidenceResponseObservationsItemEvidenceItemEight(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_problem_evidence_response_observations_item_evidence_item_eight(self) -> Option<ProblemEvidenceResponseObservationsItemEvidenceItemEight> {
        match self {
                    Self::ProblemEvidenceResponseObservationsItemEvidenceItemEight(value) => Some(value),
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

    pub fn as_problem_evidence_response_observations_item_evidence_item_since(&self) -> Option<&ProblemEvidenceResponseObservationsItemEvidenceItemSince> {
        match self {
                    Self::ProblemEvidenceResponseObservationsItemEvidenceItemSince(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_problem_evidence_response_observations_item_evidence_item_since(self) -> Option<ProblemEvidenceResponseObservationsItemEvidenceItemSince> {
        match self {
                    Self::ProblemEvidenceResponseObservationsItemEvidenceItemSince(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for ProblemEvidenceResponseObservationsItemEvidenceItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ProblemEvidenceResponseObservationsItemEvidenceItemSourceFrame(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevision(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ProblemEvidenceResponseObservationsItemEvidenceItemEndLine(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ProblemEvidenceResponseObservationsItemEvidenceItemReleaseId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ProblemEvidenceResponseObservationsItemEvidenceItemFour(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ProblemEvidenceResponseObservationsItemEvidenceItemFive(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ProblemEvidenceResponseObservationsItemEvidenceItemAffectedUsers(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ProblemEvidenceResponseObservationsItemEvidenceItemAfterLimit(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ProblemEvidenceResponseObservationsItemEvidenceItemEight(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ProblemEvidenceResponseObservationsItemEvidenceItemEvidenceDigest(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ProblemEvidenceResponseObservationsItemEvidenceItemMinSeverity(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ProblemEvidenceResponseObservationsItemEvidenceItemSince(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
