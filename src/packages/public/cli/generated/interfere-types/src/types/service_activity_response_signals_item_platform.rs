pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ServiceActivityResponseSignalsItemPlatform {
    Vercel,
    Cloudflare,
    Aws,
    Gcp,
    Azure,
    Render,
    Nodejs,
    Python,
    Go,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ServiceActivityResponseSignalsItemPlatform {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Vercel => serializer.serialize_str("vercel"),
            Self::Cloudflare => serializer.serialize_str("cloudflare"),
            Self::Aws => serializer.serialize_str("aws"),
            Self::Gcp => serializer.serialize_str("gcp"),
            Self::Azure => serializer.serialize_str("azure"),
            Self::Render => serializer.serialize_str("render"),
            Self::Nodejs => serializer.serialize_str("nodejs"),
            Self::Python => serializer.serialize_str("python"),
            Self::Go => serializer.serialize_str("go"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ServiceActivityResponseSignalsItemPlatform {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "vercel" => Ok(Self::Vercel),
            "cloudflare" => Ok(Self::Cloudflare),
            "aws" => Ok(Self::Aws),
            "gcp" => Ok(Self::Gcp),
            "azure" => Ok(Self::Azure),
            "render" => Ok(Self::Render),
            "nodejs" => Ok(Self::Nodejs),
            "python" => Ok(Self::Python),
            "go" => Ok(Self::Go),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ServiceActivityResponseSignalsItemPlatform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Vercel => write!(f, "vercel"),
            Self::Cloudflare => write!(f, "cloudflare"),
            Self::Aws => write!(f, "aws"),
            Self::Gcp => write!(f, "gcp"),
            Self::Azure => write!(f, "azure"),
            Self::Render => write!(f, "render"),
            Self::Nodejs => write!(f, "nodejs"),
            Self::Python => write!(f, "python"),
            Self::Go => write!(f, "go"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
