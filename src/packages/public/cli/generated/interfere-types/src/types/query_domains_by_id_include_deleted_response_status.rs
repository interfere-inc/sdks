pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum QueryDomainsByIdIncludeDeletedResponseStatus {
    Pending,
    Active,
    Failed,
    RequiresReactivation,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for QueryDomainsByIdIncludeDeletedResponseStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Pending => serializer.serialize_str("pending"),
            Self::Active => serializer.serialize_str("active"),
            Self::Failed => serializer.serialize_str("failed"),
            Self::RequiresReactivation => serializer.serialize_str("requires_reactivation"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for QueryDomainsByIdIncludeDeletedResponseStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "pending" => Ok(Self::Pending),
            "active" => Ok(Self::Active),
            "failed" => Ok(Self::Failed),
            "requires_reactivation" => Ok(Self::RequiresReactivation),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for QueryDomainsByIdIncludeDeletedResponseStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pending => write!(f, "pending"),
            Self::Active => write!(f, "active"),
            Self::Failed => write!(f, "failed"),
            Self::RequiresReactivation => write!(f, "requires_reactivation"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
