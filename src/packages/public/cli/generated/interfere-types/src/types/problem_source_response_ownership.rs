pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ProblemSourceResponseOwnership {
    Datasource,
    Surface,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ProblemSourceResponseOwnership {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Datasource => serializer.serialize_str("datasource"),
            Self::Surface => serializer.serialize_str("surface"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ProblemSourceResponseOwnership {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "datasource" => Ok(Self::Datasource),
            "surface" => Ok(Self::Surface),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ProblemSourceResponseOwnership {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Datasource => write!(f, "datasource"),
            Self::Surface => write!(f, "surface"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
