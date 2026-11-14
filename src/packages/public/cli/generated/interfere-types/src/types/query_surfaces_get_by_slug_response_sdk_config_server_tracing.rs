pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum QuerySurfacesGetBySlugResponseSdkConfigServerTracing {
    Full,
    Drain,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for QuerySurfacesGetBySlugResponseSdkConfigServerTracing {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Full => serializer.serialize_str("full"),
            Self::Drain => serializer.serialize_str("drain"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for QuerySurfacesGetBySlugResponseSdkConfigServerTracing {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "full" => Ok(Self::Full),
            "drain" => Ok(Self::Drain),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for QuerySurfacesGetBySlugResponseSdkConfigServerTracing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Full => write!(f, "full"),
            Self::Drain => write!(f, "drain"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
