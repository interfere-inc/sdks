pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum QueryIntegrationsInstallationsResponseItemProviderMetadataAuthenticationAuthentication {
    Bearer,
    None,
    Oauth,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for QueryIntegrationsInstallationsResponseItemProviderMetadataAuthenticationAuthentication {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Bearer => serializer.serialize_str("bearer"),
            Self::None => serializer.serialize_str("none"),
            Self::Oauth => serializer.serialize_str("oauth"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for QueryIntegrationsInstallationsResponseItemProviderMetadataAuthenticationAuthentication {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "bearer" => Ok(Self::Bearer),
            "none" => Ok(Self::None),
            "oauth" => Ok(Self::Oauth),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for QueryIntegrationsInstallationsResponseItemProviderMetadataAuthenticationAuthentication {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bearer => write!(f, "bearer"),
            Self::None => write!(f, "none"),
            Self::Oauth => write!(f, "oauth"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
