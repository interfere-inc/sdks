pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum QueryDomainsProxyInUseResponseItemType {
    Clerk,
    Proxy,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for QueryDomainsProxyInUseResponseItemType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Clerk => serializer.serialize_str("clerk"),
            Self::Proxy => serializer.serialize_str("proxy"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for QueryDomainsProxyInUseResponseItemType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "clerk" => Ok(Self::Clerk),
            "proxy" => Ok(Self::Proxy),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for QueryDomainsProxyInUseResponseItemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Clerk => write!(f, "clerk"),
            Self::Proxy => write!(f, "proxy"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
