pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum QueryIntegrationsSlackInstallationResponseProviderMetadataChatRoutingChatRoutingValueThreshold {
    High,
    Medium,
    All,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for QueryIntegrationsSlackInstallationResponseProviderMetadataChatRoutingChatRoutingValueThreshold {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::High => serializer.serialize_str("HIGH"),
            Self::Medium => serializer.serialize_str("MEDIUM"),
            Self::All => serializer.serialize_str("ALL"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for QueryIntegrationsSlackInstallationResponseProviderMetadataChatRoutingChatRoutingValueThreshold {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "HIGH" => Ok(Self::High),
            "MEDIUM" => Ok(Self::Medium),
            "ALL" => Ok(Self::All),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for QueryIntegrationsSlackInstallationResponseProviderMetadataChatRoutingChatRoutingValueThreshold {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::High => write!(f, "HIGH"),
            Self::Medium => write!(f, "MEDIUM"),
            Self::All => write!(f, "ALL"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
