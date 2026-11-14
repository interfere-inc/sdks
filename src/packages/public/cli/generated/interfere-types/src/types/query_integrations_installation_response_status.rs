pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum QueryIntegrationsInstallationResponseStatus {
    Connected,
    Disconnected,
    RequiresReauth,
    Pending,
    ActionRequired,
    Error,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for QueryIntegrationsInstallationResponseStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Connected => serializer.serialize_str("connected"),
            Self::Disconnected => serializer.serialize_str("disconnected"),
            Self::RequiresReauth => serializer.serialize_str("requires_reauth"),
            Self::Pending => serializer.serialize_str("pending"),
            Self::ActionRequired => serializer.serialize_str("action_required"),
            Self::Error => serializer.serialize_str("error"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for QueryIntegrationsInstallationResponseStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "connected" => Ok(Self::Connected),
            "disconnected" => Ok(Self::Disconnected),
            "requires_reauth" => Ok(Self::RequiresReauth),
            "pending" => Ok(Self::Pending),
            "action_required" => Ok(Self::ActionRequired),
            "error" => Ok(Self::Error),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for QueryIntegrationsInstallationResponseStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Connected => write!(f, "connected"),
            Self::Disconnected => write!(f, "disconnected"),
            Self::RequiresReauth => write!(f, "requires_reauth"),
            Self::Pending => write!(f, "pending"),
            Self::ActionRequired => write!(f, "action_required"),
            Self::Error => write!(f, "error"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
