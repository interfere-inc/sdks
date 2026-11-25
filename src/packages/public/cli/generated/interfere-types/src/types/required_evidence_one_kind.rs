pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum RequiredEvidenceOneKind {
        RequiredEvidenceOneKindZero(RequiredEvidenceOneKindZero),

        RequiredEvidenceOneKindOne(RequiredEvidenceOneKindOne),
}

impl RequiredEvidenceOneKind {
    pub fn is_required_evidence_one_kind_zero(&self) -> bool {
        matches!(self, Self::RequiredEvidenceOneKindZero(_))
    }

    pub fn is_required_evidence_one_kind_one(&self) -> bool {
        matches!(self, Self::RequiredEvidenceOneKindOne(_))
    }


    pub fn as_required_evidence_one_kind_zero(&self) -> Option<&RequiredEvidenceOneKindZero> {
        match self {
                    Self::RequiredEvidenceOneKindZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_required_evidence_one_kind_zero(self) -> Option<RequiredEvidenceOneKindZero> {
        match self {
                    Self::RequiredEvidenceOneKindZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_required_evidence_one_kind_one(&self) -> Option<&RequiredEvidenceOneKindOne> {
        match self {
                    Self::RequiredEvidenceOneKindOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_required_evidence_one_kind_one(self) -> Option<RequiredEvidenceOneKindOne> {
        match self {
                    Self::RequiredEvidenceOneKindOne(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for RequiredEvidenceOneKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RequiredEvidenceOneKindZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::RequiredEvidenceOneKindOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
