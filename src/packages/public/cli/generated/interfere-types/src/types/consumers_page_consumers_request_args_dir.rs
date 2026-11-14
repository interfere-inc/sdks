pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PageConsumersRequestArgsDir {
    Forward,
    Backward,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PageConsumersRequestArgsDir {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Forward => serializer.serialize_str("forward"),
            Self::Backward => serializer.serialize_str("backward"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PageConsumersRequestArgsDir {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "forward" => Ok(Self::Forward),
            "backward" => Ok(Self::Backward),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PageConsumersRequestArgsDir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Forward => write!(f, "forward"),
            Self::Backward => write!(f, "backward"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
