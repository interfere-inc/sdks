pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// ManifestBundler
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ManifestBundler {
    Webpack,
    Turbopack,
    Rollup,
    Rolldown,
    Tsc,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ManifestBundler {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Webpack => serializer.serialize_str("webpack"),
            Self::Turbopack => serializer.serialize_str("turbopack"),
            Self::Rollup => serializer.serialize_str("rollup"),
            Self::Rolldown => serializer.serialize_str("rolldown"),
            Self::Tsc => serializer.serialize_str("tsc"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ManifestBundler {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "webpack" => Ok(Self::Webpack),
            "turbopack" => Ok(Self::Turbopack),
            "rollup" => Ok(Self::Rollup),
            "rolldown" => Ok(Self::Rolldown),
            "tsc" => Ok(Self::Tsc),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ManifestBundler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Webpack => write!(f, "webpack"),
            Self::Turbopack => write!(f, "turbopack"),
            Self::Rollup => write!(f, "rollup"),
            Self::Rolldown => write!(f, "rolldown"),
            Self::Tsc => write!(f, "tsc"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
