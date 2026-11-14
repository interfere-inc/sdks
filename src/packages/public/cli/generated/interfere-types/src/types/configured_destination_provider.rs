pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// ConfiguredDestinationProvider
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConfiguredDestinationProvider {
    Vercel,
    Cloudflare,
    Cli,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ConfiguredDestinationProvider {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Vercel => serializer.serialize_str("vercel"),
            Self::Cloudflare => serializer.serialize_str("cloudflare"),
            Self::Cli => serializer.serialize_str("cli"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ConfiguredDestinationProvider {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "vercel" => Ok(Self::Vercel),
            "cloudflare" => Ok(Self::Cloudflare),
            "cli" => Ok(Self::Cli),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ConfiguredDestinationProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Vercel => write!(f, "vercel"),
            Self::Cloudflare => write!(f, "cloudflare"),
            Self::Cli => write!(f, "cli"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
