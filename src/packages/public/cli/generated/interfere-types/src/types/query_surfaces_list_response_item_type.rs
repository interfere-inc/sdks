pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum QuerySurfacesListResponseItemType {
    Elysia,
    Nest,
    Nextjs,
    Python,
    React,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for QuerySurfacesListResponseItemType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Elysia => serializer.serialize_str("elysia"),
            Self::Nest => serializer.serialize_str("nest"),
            Self::Nextjs => serializer.serialize_str("nextjs"),
            Self::Python => serializer.serialize_str("python"),
            Self::React => serializer.serialize_str("react"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for QuerySurfacesListResponseItemType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "elysia" => Ok(Self::Elysia),
            "nest" => Ok(Self::Nest),
            "nextjs" => Ok(Self::Nextjs),
            "python" => Ok(Self::Python),
            "react" => Ok(Self::React),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for QuerySurfacesListResponseItemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Elysia => write!(f, "elysia"),
            Self::Nest => write!(f, "nest"),
            Self::Nextjs => write!(f, "nextjs"),
            Self::Python => write!(f, "python"),
            Self::React => write!(f, "react"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
