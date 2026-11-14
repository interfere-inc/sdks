pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TimelineEventContentCategoryCategory {
    RuntimeError,
    Network,
    Performance,
    Security,
    Ui,
    Data,
    Auth,
    Integration,
    Infrastructure,
    Other,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TimelineEventContentCategoryCategory {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::RuntimeError => serializer.serialize_str("RUNTIME_ERROR"),
            Self::Network => serializer.serialize_str("NETWORK"),
            Self::Performance => serializer.serialize_str("PERFORMANCE"),
            Self::Security => serializer.serialize_str("SECURITY"),
            Self::Ui => serializer.serialize_str("UI"),
            Self::Data => serializer.serialize_str("DATA"),
            Self::Auth => serializer.serialize_str("AUTH"),
            Self::Integration => serializer.serialize_str("INTEGRATION"),
            Self::Infrastructure => serializer.serialize_str("INFRASTRUCTURE"),
            Self::Other => serializer.serialize_str("OTHER"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TimelineEventContentCategoryCategory {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "RUNTIME_ERROR" => Ok(Self::RuntimeError),
            "NETWORK" => Ok(Self::Network),
            "PERFORMANCE" => Ok(Self::Performance),
            "SECURITY" => Ok(Self::Security),
            "UI" => Ok(Self::Ui),
            "DATA" => Ok(Self::Data),
            "AUTH" => Ok(Self::Auth),
            "INTEGRATION" => Ok(Self::Integration),
            "INFRASTRUCTURE" => Ok(Self::Infrastructure),
            "OTHER" => Ok(Self::Other),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TimelineEventContentCategoryCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RuntimeError => write!(f, "RUNTIME_ERROR"),
            Self::Network => write!(f, "NETWORK"),
            Self::Performance => write!(f, "PERFORMANCE"),
            Self::Security => write!(f, "SECURITY"),
            Self::Ui => write!(f, "UI"),
            Self::Data => write!(f, "DATA"),
            Self::Auth => write!(f, "AUTH"),
            Self::Integration => write!(f, "INTEGRATION"),
            Self::Infrastructure => write!(f, "INFRASTRUCTURE"),
            Self::Other => write!(f, "OTHER"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
