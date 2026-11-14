pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DetectIntegrationsResponseIntegrationsItemInstallConnectionId {
    Github,
    GheCloud,
    GheServer,
    Vercel,
    Cloudflare,
    Slack,
    Linear,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DetectIntegrationsResponseIntegrationsItemInstallConnectionId {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Github => serializer.serialize_str("github"),
            Self::GheCloud => serializer.serialize_str("ghe_cloud"),
            Self::GheServer => serializer.serialize_str("ghe_server"),
            Self::Vercel => serializer.serialize_str("vercel"),
            Self::Cloudflare => serializer.serialize_str("cloudflare"),
            Self::Slack => serializer.serialize_str("slack"),
            Self::Linear => serializer.serialize_str("linear"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DetectIntegrationsResponseIntegrationsItemInstallConnectionId {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "github" => Ok(Self::Github),
            "ghe_cloud" => Ok(Self::GheCloud),
            "ghe_server" => Ok(Self::GheServer),
            "vercel" => Ok(Self::Vercel),
            "cloudflare" => Ok(Self::Cloudflare),
            "slack" => Ok(Self::Slack),
            "linear" => Ok(Self::Linear),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for DetectIntegrationsResponseIntegrationsItemInstallConnectionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Github => write!(f, "github"),
            Self::GheCloud => write!(f, "ghe_cloud"),
            Self::GheServer => write!(f, "ghe_server"),
            Self::Vercel => write!(f, "vercel"),
            Self::Cloudflare => write!(f, "cloudflare"),
            Self::Slack => write!(f, "slack"),
            Self::Linear => write!(f, "linear"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
