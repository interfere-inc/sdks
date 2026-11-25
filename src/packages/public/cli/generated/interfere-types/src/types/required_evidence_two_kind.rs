pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum RequiredEvidenceTwoKind {
        RequiredEvidenceTwoKindZero(RequiredEvidenceTwoKindZero),

        RequiredEvidenceTwoKindOne(RequiredEvidenceTwoKindOne),
}

impl RequiredEvidenceTwoKind {
    pub fn is_required_evidence_two_kind_zero(&self) -> bool {
        matches!(self, Self::RequiredEvidenceTwoKindZero(_))
    }

    pub fn is_required_evidence_two_kind_one(&self) -> bool {
        matches!(self, Self::RequiredEvidenceTwoKindOne(_))
    }


    pub fn as_required_evidence_two_kind_zero(&self) -> Option<&RequiredEvidenceTwoKindZero> {
        match self {
                    Self::RequiredEvidenceTwoKindZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_required_evidence_two_kind_zero(self) -> Option<RequiredEvidenceTwoKindZero> {
        match self {
                    Self::RequiredEvidenceTwoKindZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_required_evidence_two_kind_one(&self) -> Option<&RequiredEvidenceTwoKindOne> {
        match self {
                    Self::RequiredEvidenceTwoKindOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_required_evidence_two_kind_one(self) -> Option<RequiredEvidenceTwoKindOne> {
        match self {
                    Self::RequiredEvidenceTwoKindOne(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for RequiredEvidenceTwoKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RequiredEvidenceTwoKindZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::RequiredEvidenceTwoKindOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
