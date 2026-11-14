pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateBillingPortalRequestFlow {
    PaymentMethodUpdate,
    SubscriptionCancel,
    SubscriptionUpdate,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateBillingPortalRequestFlow {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::PaymentMethodUpdate => serializer.serialize_str("payment_method_update"),
            Self::SubscriptionCancel => serializer.serialize_str("subscription_cancel"),
            Self::SubscriptionUpdate => serializer.serialize_str("subscription_update"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateBillingPortalRequestFlow {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "payment_method_update" => Ok(Self::PaymentMethodUpdate),
            "subscription_cancel" => Ok(Self::SubscriptionCancel),
            "subscription_update" => Ok(Self::SubscriptionUpdate),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateBillingPortalRequestFlow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PaymentMethodUpdate => write!(f, "payment_method_update"),
            Self::SubscriptionCancel => write!(f, "subscription_cancel"),
            Self::SubscriptionUpdate => write!(f, "subscription_update"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
