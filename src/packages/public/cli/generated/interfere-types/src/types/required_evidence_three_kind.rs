pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum RequiredEvidenceThreeKind {
        RequiredEvidenceThreeKindZero(RequiredEvidenceThreeKindZero),

        RequiredEvidenceThreeKindOne(RequiredEvidenceThreeKindOne),
}

impl RequiredEvidenceThreeKind {
    pub fn is_required_evidence_three_kind_zero(&self) -> bool {
        matches!(self, Self::RequiredEvidenceThreeKindZero(_))
    }

    pub fn is_required_evidence_three_kind_one(&self) -> bool {
        matches!(self, Self::RequiredEvidenceThreeKindOne(_))
    }


    pub fn as_required_evidence_three_kind_zero(&self) -> Option<&RequiredEvidenceThreeKindZero> {
        match self {
                    Self::RequiredEvidenceThreeKindZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_required_evidence_three_kind_zero(self) -> Option<RequiredEvidenceThreeKindZero> {
        match self {
                    Self::RequiredEvidenceThreeKindZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_required_evidence_three_kind_one(&self) -> Option<&RequiredEvidenceThreeKindOne> {
        match self {
                    Self::RequiredEvidenceThreeKindOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_required_evidence_three_kind_one(self) -> Option<RequiredEvidenceThreeKindOne> {
        match self {
                    Self::RequiredEvidenceThreeKindOne(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for RequiredEvidenceThreeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RequiredEvidenceThreeKindZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::RequiredEvidenceThreeKindOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
