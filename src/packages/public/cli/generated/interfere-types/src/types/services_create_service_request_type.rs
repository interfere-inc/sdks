pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The kind of data source the user chose. It is stored with type_source user, so telemetry never changes it, and it replaces a type inferred from earlier telemetry.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateServiceRequestType {
    Other,
    Backend,
    Website,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateServiceRequestType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Other => serializer.serialize_str("other"),
            Self::Backend => serializer.serialize_str("backend"),
            Self::Website => serializer.serialize_str("website"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateServiceRequestType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "other" => Ok(Self::Other),
            "backend" => Ok(Self::Backend),
            "website" => Ok(Self::Website),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateServiceRequestType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Other => write!(f, "other"),
            Self::Backend => write!(f, "backend"),
            Self::Website => write!(f, "website"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
