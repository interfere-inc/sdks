pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationBasisBasis {
    Contradicted,
    UnsupportedWithoutContradiction,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationBasisBasis {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Contradicted => serializer.serialize_str("contradicted"),
            Self::UnsupportedWithoutContradiction => serializer.serialize_str("unsupported-without-contradiction"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationBasisBasis {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "contradicted" => Ok(Self::Contradicted),
            "unsupported-without-contradiction" => Ok(Self::UnsupportedWithoutContradiction),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationBasisBasis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Contradicted => write!(f, "contradicted"),
            Self::UnsupportedWithoutContradiction => write!(f, "unsupported-without-contradiction"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
