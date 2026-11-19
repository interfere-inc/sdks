pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CreateBillingCheckoutRequestPlanOne {
    #[serde(rename = "business")]
    Business,
}
impl fmt::Display for CreateBillingCheckoutRequestPlanOne {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Business => "business",
        };
        write!(f, "{}", s)
    }
}
