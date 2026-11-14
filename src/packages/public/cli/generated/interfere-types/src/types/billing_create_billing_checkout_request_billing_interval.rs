pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateBillingCheckoutRequestBillingInterval {
    Monthly,
    Yearly,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateBillingCheckoutRequestBillingInterval {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Monthly => serializer.serialize_str("monthly"),
            Self::Yearly => serializer.serialize_str("yearly"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateBillingCheckoutRequestBillingInterval {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "monthly" => Ok(Self::Monthly),
            "yearly" => Ok(Self::Yearly),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateBillingCheckoutRequestBillingInterval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Monthly => write!(f, "monthly"),
            Self::Yearly => write!(f, "yearly"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
