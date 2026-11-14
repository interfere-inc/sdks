pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperation {
        ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationCursor(ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationCursor),

        ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationArguments(ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationArguments),

        ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationKind(ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationKind),
}

impl ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperation {
    pub fn is_problem_evidence_response_observations_item_evidence_item_connection_revision_operation_cursor(&self) -> bool {
        matches!(self, Self::ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationCursor(_))
    }

    pub fn is_problem_evidence_response_observations_item_evidence_item_connection_revision_operation_arguments(&self) -> bool {
        matches!(self, Self::ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationArguments(_))
    }

    pub fn is_problem_evidence_response_observations_item_evidence_item_connection_revision_operation_kind(&self) -> bool {
        matches!(self, Self::ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationKind(_))
    }


    pub fn as_problem_evidence_response_observations_item_evidence_item_connection_revision_operation_cursor(&self) -> Option<&ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationCursor> {
        match self {
                    Self::ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationCursor(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_problem_evidence_response_observations_item_evidence_item_connection_revision_operation_cursor(self) -> Option<ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationCursor> {
        match self {
                    Self::ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationCursor(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_problem_evidence_response_observations_item_evidence_item_connection_revision_operation_arguments(&self) -> Option<&ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationArguments> {
        match self {
                    Self::ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationArguments(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_problem_evidence_response_observations_item_evidence_item_connection_revision_operation_arguments(self) -> Option<ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationArguments> {
        match self {
                    Self::ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationArguments(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_problem_evidence_response_observations_item_evidence_item_connection_revision_operation_kind(&self) -> Option<&ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationKind> {
        match self {
                    Self::ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationKind(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_problem_evidence_response_observations_item_evidence_item_connection_revision_operation_kind(self) -> Option<ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationKind> {
        match self {
                    Self::ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationKind(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationCursor(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationArguments(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationKind(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
