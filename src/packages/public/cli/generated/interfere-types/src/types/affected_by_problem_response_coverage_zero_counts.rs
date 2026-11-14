pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AffectedByProblemResponseCoverageZeroCounts {
    Observed,
    Unavailable,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AffectedByProblemResponseCoverageZeroCounts {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Observed => serializer.serialize_str("observed"),
            Self::Unavailable => serializer.serialize_str("unavailable"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AffectedByProblemResponseCoverageZeroCounts {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "observed" => Ok(Self::Observed),
            "unavailable" => Ok(Self::Unavailable),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AffectedByProblemResponseCoverageZeroCounts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Observed => write!(f, "observed"),
            Self::Unavailable => write!(f, "unavailable"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
