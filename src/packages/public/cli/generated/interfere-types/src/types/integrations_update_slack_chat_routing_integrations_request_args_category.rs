pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UpdateSlackChatRoutingIntegrationsRequestArgsCategory {
    Problems,
    Team,
    Collaboration,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UpdateSlackChatRoutingIntegrationsRequestArgsCategory {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Problems => serializer.serialize_str("problems"),
            Self::Team => serializer.serialize_str("team"),
            Self::Collaboration => serializer.serialize_str("collaboration"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UpdateSlackChatRoutingIntegrationsRequestArgsCategory {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "problems" => Ok(Self::Problems),
            "team" => Ok(Self::Team),
            "collaboration" => Ok(Self::Collaboration),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UpdateSlackChatRoutingIntegrationsRequestArgsCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Problems => write!(f, "problems"),
            Self::Team => write!(f, "team"),
            Self::Collaboration => write!(f, "collaboration"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
