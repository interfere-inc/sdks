pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum RequiredEvidence {
        RequiredEvidenceZero(RequiredEvidenceZero),

        RequiredEvidenceOne(RequiredEvidenceOne),

        RequiredEvidenceTwo(RequiredEvidenceTwo),

        RequiredEvidenceThree(RequiredEvidenceThree),
}

impl RequiredEvidence {
    pub fn is_required_evidence_zero(&self) -> bool {
        matches!(self, Self::RequiredEvidenceZero(_))
    }

    pub fn is_required_evidence_one(&self) -> bool {
        matches!(self, Self::RequiredEvidenceOne(_))
    }

    pub fn is_required_evidence_two(&self) -> bool {
        matches!(self, Self::RequiredEvidenceTwo(_))
    }

    pub fn is_required_evidence_three(&self) -> bool {
        matches!(self, Self::RequiredEvidenceThree(_))
    }


    pub fn as_required_evidence_zero(&self) -> Option<&RequiredEvidenceZero> {
        match self {
                    Self::RequiredEvidenceZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_required_evidence_zero(self) -> Option<RequiredEvidenceZero> {
        match self {
                    Self::RequiredEvidenceZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_required_evidence_one(&self) -> Option<&RequiredEvidenceOne> {
        match self {
                    Self::RequiredEvidenceOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_required_evidence_one(self) -> Option<RequiredEvidenceOne> {
        match self {
                    Self::RequiredEvidenceOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_required_evidence_two(&self) -> Option<&RequiredEvidenceTwo> {
        match self {
                    Self::RequiredEvidenceTwo(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_required_evidence_two(self) -> Option<RequiredEvidenceTwo> {
        match self {
                    Self::RequiredEvidenceTwo(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_required_evidence_three(&self) -> Option<&RequiredEvidenceThree> {
        match self {
                    Self::RequiredEvidenceThree(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_required_evidence_three(self) -> Option<RequiredEvidenceThree> {
        match self {
                    Self::RequiredEvidenceThree(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for RequiredEvidence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RequiredEvidenceZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::RequiredEvidenceOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::RequiredEvidenceTwo(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::RequiredEvidenceThree(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
