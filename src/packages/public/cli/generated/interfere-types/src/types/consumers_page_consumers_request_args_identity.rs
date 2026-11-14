pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PageConsumersRequestArgsIdentity {
    Anonymous,
    Identified,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PageConsumersRequestArgsIdentity {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Anonymous => serializer.serialize_str("anonymous"),
            Self::Identified => serializer.serialize_str("identified"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PageConsumersRequestArgsIdentity {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "anonymous" => Ok(Self::Anonymous),
            "identified" => Ok(Self::Identified),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PageConsumersRequestArgsIdentity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Anonymous => write!(f, "anonymous"),
            Self::Identified => write!(f, "identified"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
