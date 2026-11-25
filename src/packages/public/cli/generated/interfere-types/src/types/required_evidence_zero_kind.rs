pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum RequiredEvidenceZeroKind {
        RequiredEvidenceZeroKindZero(RequiredEvidenceZeroKindZero),

        RequiredEvidenceZeroKindOne(RequiredEvidenceZeroKindOne),
}

impl RequiredEvidenceZeroKind {
    pub fn is_required_evidence_zero_kind_zero(&self) -> bool {
        matches!(self, Self::RequiredEvidenceZeroKindZero(_))
    }

    pub fn is_required_evidence_zero_kind_one(&self) -> bool {
        matches!(self, Self::RequiredEvidenceZeroKindOne(_))
    }


    pub fn as_required_evidence_zero_kind_zero(&self) -> Option<&RequiredEvidenceZeroKindZero> {
        match self {
                    Self::RequiredEvidenceZeroKindZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_required_evidence_zero_kind_zero(self) -> Option<RequiredEvidenceZeroKindZero> {
        match self {
                    Self::RequiredEvidenceZeroKindZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_required_evidence_zero_kind_one(&self) -> Option<&RequiredEvidenceZeroKindOne> {
        match self {
                    Self::RequiredEvidenceZeroKindOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_required_evidence_zero_kind_one(self) -> Option<RequiredEvidenceZeroKindOne> {
        match self {
                    Self::RequiredEvidenceZeroKindOne(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for RequiredEvidenceZeroKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RequiredEvidenceZeroKindZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::RequiredEvidenceZeroKindOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
