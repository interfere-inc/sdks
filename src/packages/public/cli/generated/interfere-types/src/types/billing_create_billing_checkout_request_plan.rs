pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum CreateBillingCheckoutRequestPlan {
        CreateBillingCheckoutRequestPlanZero(CreateBillingCheckoutRequestPlanZero),

        CreateBillingCheckoutRequestPlanOne(CreateBillingCheckoutRequestPlanOne),
}

impl CreateBillingCheckoutRequestPlan {
    pub fn is_create_billing_checkout_request_plan_zero(&self) -> bool {
        matches!(self, Self::CreateBillingCheckoutRequestPlanZero(_))
    }

    pub fn is_create_billing_checkout_request_plan_one(&self) -> bool {
        matches!(self, Self::CreateBillingCheckoutRequestPlanOne(_))
    }


    pub fn as_create_billing_checkout_request_plan_zero(&self) -> Option<&CreateBillingCheckoutRequestPlanZero> {
        match self {
                    Self::CreateBillingCheckoutRequestPlanZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_create_billing_checkout_request_plan_zero(self) -> Option<CreateBillingCheckoutRequestPlanZero> {
        match self {
                    Self::CreateBillingCheckoutRequestPlanZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_create_billing_checkout_request_plan_one(&self) -> Option<&CreateBillingCheckoutRequestPlanOne> {
        match self {
                    Self::CreateBillingCheckoutRequestPlanOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_create_billing_checkout_request_plan_one(self) -> Option<CreateBillingCheckoutRequestPlanOne> {
        match self {
                    Self::CreateBillingCheckoutRequestPlanOne(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for CreateBillingCheckoutRequestPlan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CreateBillingCheckoutRequestPlanZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::CreateBillingCheckoutRequestPlanOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
