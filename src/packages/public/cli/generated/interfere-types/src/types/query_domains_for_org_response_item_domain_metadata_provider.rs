pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum QueryDomainsForOrgResponseItemDomainMetadataProvider {
    Cloudflare,
    Aws,
    Vercel,
    Godaddy,
    Namecheap,
    Squarespace,
    GoogleCloud,
    Generic,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for QueryDomainsForOrgResponseItemDomainMetadataProvider {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Cloudflare => serializer.serialize_str("cloudflare"),
            Self::Aws => serializer.serialize_str("aws"),
            Self::Vercel => serializer.serialize_str("vercel"),
            Self::Godaddy => serializer.serialize_str("godaddy"),
            Self::Namecheap => serializer.serialize_str("namecheap"),
            Self::Squarespace => serializer.serialize_str("squarespace"),
            Self::GoogleCloud => serializer.serialize_str("google-cloud"),
            Self::Generic => serializer.serialize_str("generic"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for QueryDomainsForOrgResponseItemDomainMetadataProvider {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "cloudflare" => Ok(Self::Cloudflare),
            "aws" => Ok(Self::Aws),
            "vercel" => Ok(Self::Vercel),
            "godaddy" => Ok(Self::Godaddy),
            "namecheap" => Ok(Self::Namecheap),
            "squarespace" => Ok(Self::Squarespace),
            "google-cloud" => Ok(Self::GoogleCloud),
            "generic" => Ok(Self::Generic),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for QueryDomainsForOrgResponseItemDomainMetadataProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Cloudflare => write!(f, "cloudflare"),
            Self::Aws => write!(f, "aws"),
            Self::Vercel => write!(f, "vercel"),
            Self::Godaddy => write!(f, "godaddy"),
            Self::Namecheap => write!(f, "namecheap"),
            Self::Squarespace => write!(f, "squarespace"),
            Self::GoogleCloud => write!(f, "google-cloud"),
            Self::Generic => write!(f, "generic"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
