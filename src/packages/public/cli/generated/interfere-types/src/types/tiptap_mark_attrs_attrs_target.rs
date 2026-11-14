pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TiptapMarkAttrsAttrsTarget {
    Blank,
    Self_,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TiptapMarkAttrsAttrsTarget {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Blank => serializer.serialize_str("_blank"),
            Self::Self_ => serializer.serialize_str("_self"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TiptapMarkAttrsAttrsTarget {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "_blank" => Ok(Self::Blank),
            "_self" => Ok(Self::Self_),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TiptapMarkAttrsAttrsTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Blank => write!(f, "_blank"),
            Self::Self_ => write!(f, "_self"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
