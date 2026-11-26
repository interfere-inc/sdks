pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// When sessions without an identified user record replay: always, only once an error occurs, or never.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum QueryServicesByIdResponseWebsiteSettingsAnonymousReplayPolicy {
    Always,
    OnError,
    Never,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for QueryServicesByIdResponseWebsiteSettingsAnonymousReplayPolicy {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Always => serializer.serialize_str("always"),
            Self::OnError => serializer.serialize_str("on_error"),
            Self::Never => serializer.serialize_str("never"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for QueryServicesByIdResponseWebsiteSettingsAnonymousReplayPolicy {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "always" => Ok(Self::Always),
            "on_error" => Ok(Self::OnError),
            "never" => Ok(Self::Never),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for QueryServicesByIdResponseWebsiteSettingsAnonymousReplayPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Always => write!(f, "always"),
            Self::OnError => write!(f, "on_error"),
            Self::Never => write!(f, "never"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
