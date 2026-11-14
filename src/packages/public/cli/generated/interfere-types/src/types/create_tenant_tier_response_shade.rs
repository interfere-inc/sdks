pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateTenantTierResponseShade {
    Blue,
    Gray,
    Grass,
    Purple,
    Red,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateTenantTierResponseShade {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Blue => serializer.serialize_str("blue"),
            Self::Gray => serializer.serialize_str("gray"),
            Self::Grass => serializer.serialize_str("grass"),
            Self::Purple => serializer.serialize_str("purple"),
            Self::Red => serializer.serialize_str("red"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateTenantTierResponseShade {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "blue" => Ok(Self::Blue),
            "gray" => Ok(Self::Gray),
            "grass" => Ok(Self::Grass),
            "purple" => Ok(Self::Purple),
            "red" => Ok(Self::Red),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateTenantTierResponseShade {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Blue => write!(f, "blue"),
            Self::Gray => write!(f, "gray"),
            Self::Grass => write!(f, "grass"),
            Self::Purple => write!(f, "purple"),
            Self::Red => write!(f, "red"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
