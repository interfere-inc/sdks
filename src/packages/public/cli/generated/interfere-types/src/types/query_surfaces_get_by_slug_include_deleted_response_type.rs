pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum QuerySurfacesGetBySlugIncludeDeletedResponseType {
    Nextjs,
    React,
    Generic,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for QuerySurfacesGetBySlugIncludeDeletedResponseType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Nextjs => serializer.serialize_str("nextjs"),
            Self::React => serializer.serialize_str("react"),
            Self::Generic => serializer.serialize_str("generic"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for QuerySurfacesGetBySlugIncludeDeletedResponseType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "nextjs" => Ok(Self::Nextjs),
            "react" => Ok(Self::React),
            "generic" => Ok(Self::Generic),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for QuerySurfacesGetBySlugIncludeDeletedResponseType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nextjs => write!(f, "nextjs"),
            Self::React => write!(f, "react"),
            Self::Generic => write!(f, "generic"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
