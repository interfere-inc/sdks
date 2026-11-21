pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// What still refers to the service: integration (a destination resource), apps, keys (public keys), sessions (replay sessions), findings or problems.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConflictErrorBodyZeroDataBlockersItem {
    Integration,
    Apps,
    Keys,
    Sessions,
    Findings,
    Problems,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ConflictErrorBodyZeroDataBlockersItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Integration => serializer.serialize_str("integration"),
            Self::Apps => serializer.serialize_str("apps"),
            Self::Keys => serializer.serialize_str("keys"),
            Self::Sessions => serializer.serialize_str("sessions"),
            Self::Findings => serializer.serialize_str("findings"),
            Self::Problems => serializer.serialize_str("problems"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ConflictErrorBodyZeroDataBlockersItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "integration" => Ok(Self::Integration),
            "apps" => Ok(Self::Apps),
            "keys" => Ok(Self::Keys),
            "sessions" => Ok(Self::Sessions),
            "findings" => Ok(Self::Findings),
            "problems" => Ok(Self::Problems),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ConflictErrorBodyZeroDataBlockersItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Integration => write!(f, "integration"),
            Self::Apps => write!(f, "apps"),
            Self::Keys => write!(f, "keys"),
            Self::Sessions => write!(f, "sessions"),
            Self::Findings => write!(f, "findings"),
            Self::Problems => write!(f, "problems"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
