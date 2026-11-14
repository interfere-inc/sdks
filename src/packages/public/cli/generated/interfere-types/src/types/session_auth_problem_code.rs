pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SessionAuthProblemCode {
    AuthMissing,
    AuthSessionInvalid,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SessionAuthProblemCode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AuthMissing => serializer.serialize_str("AUTH_MISSING"),
            Self::AuthSessionInvalid => serializer.serialize_str("AUTH_SESSION_INVALID"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SessionAuthProblemCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "AUTH_MISSING" => Ok(Self::AuthMissing),
            "AUTH_SESSION_INVALID" => Ok(Self::AuthSessionInvalid),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SessionAuthProblemCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AuthMissing => write!(f, "AUTH_MISSING"),
            Self::AuthSessionInvalid => write!(f, "AUTH_SESSION_INVALID"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
