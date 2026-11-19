pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateBillingCheckoutRequestPlanZero {
    Pro,
    Team,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateBillingCheckoutRequestPlanZero {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Pro => serializer.serialize_str("pro"),
            Self::Team => serializer.serialize_str("team"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateBillingCheckoutRequestPlanZero {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "pro" => Ok(Self::Pro),
            "team" => Ok(Self::Team),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateBillingCheckoutRequestPlanZero {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pro => write!(f, "pro"),
            Self::Team => write!(f, "team"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
