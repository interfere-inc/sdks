pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AuthProblemCode {
    AuthMissing,
    AuthPkInvalid,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AuthProblemCode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AuthMissing => serializer.serialize_str("AUTH_MISSING"),
            Self::AuthPkInvalid => serializer.serialize_str("AUTH_PK_INVALID"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AuthProblemCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "AUTH_MISSING" => Ok(Self::AuthMissing),
            "AUTH_PK_INVALID" => Ok(Self::AuthPkInvalid),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AuthProblemCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AuthMissing => write!(f, "AUTH_MISSING"),
            Self::AuthPkInvalid => write!(f, "AUTH_PK_INVALID"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
