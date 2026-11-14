pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionProvenance {
        ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionProvenanceConnectionId(ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionProvenanceConnectionId),

        Value(serde_json::Value),
}

impl ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionProvenance {
    pub fn is_problem_evidence_response_observations_item_evidence_item_connection_revision_provenance_connection_id(&self) -> bool {
        matches!(self, Self::ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionProvenanceConnectionId(_))
    }

    pub fn is_value(&self) -> bool {
        matches!(self, Self::Value(_))
    }


    pub fn as_problem_evidence_response_observations_item_evidence_item_connection_revision_provenance_connection_id(&self) -> Option<&ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionProvenanceConnectionId> {
        match self {
                    Self::ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionProvenanceConnectionId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_problem_evidence_response_observations_item_evidence_item_connection_revision_provenance_connection_id(self) -> Option<ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionProvenanceConnectionId> {
        match self {
                    Self::ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionProvenanceConnectionId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_value(&self) -> Option<&serde_json::Value> {
        match self {
                    Self::Value(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_value(self) -> Option<serde_json::Value> {
        match self {
                    Self::Value(value) => Some(value),
                    _ => None,
                }
    }
}
