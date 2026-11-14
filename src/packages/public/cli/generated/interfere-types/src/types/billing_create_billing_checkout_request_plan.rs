pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateBillingCheckoutRequestPlan {
    Pro,
    Growth,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateBillingCheckoutRequestPlan {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Pro => serializer.serialize_str("pro"),
            Self::Growth => serializer.serialize_str("growth"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateBillingCheckoutRequestPlan {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "pro" => Ok(Self::Pro),
            "growth" => Ok(Self::Growth),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateBillingCheckoutRequestPlan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pro => write!(f, "pro"),
            Self::Growth => write!(f, "growth"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
