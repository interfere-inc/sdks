pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SeenUserTenantsRequestArgsEdge {
    First,
    Last,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SeenUserTenantsRequestArgsEdge {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::First => serializer.serialize_str("first"),
            Self::Last => serializer.serialize_str("last"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SeenUserTenantsRequestArgsEdge {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "first" => Ok(Self::First),
            "last" => Ok(Self::Last),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SeenUserTenantsRequestArgsEdge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::First => write!(f, "first"),
            Self::Last => write!(f, "last"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
