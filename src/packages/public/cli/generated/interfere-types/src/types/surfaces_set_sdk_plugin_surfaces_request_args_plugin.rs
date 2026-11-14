pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SetSdkPluginSurfacesRequestArgsPlugin {
    Replay,
    Logs,
    PageEvents,
    RageClick,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SetSdkPluginSurfacesRequestArgsPlugin {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Replay => serializer.serialize_str("replay"),
            Self::Logs => serializer.serialize_str("logs"),
            Self::PageEvents => serializer.serialize_str("pageEvents"),
            Self::RageClick => serializer.serialize_str("rageClick"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SetSdkPluginSurfacesRequestArgsPlugin {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "replay" => Ok(Self::Replay),
            "logs" => Ok(Self::Logs),
            "pageEvents" => Ok(Self::PageEvents),
            "rageClick" => Ok(Self::RageClick),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SetSdkPluginSurfacesRequestArgsPlugin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Replay => write!(f, "replay"),
            Self::Logs => write!(f, "logs"),
            Self::PageEvents => write!(f, "pageEvents"),
            Self::RageClick => write!(f, "rageClick"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
