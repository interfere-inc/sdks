pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum UnprocessableEntityErrorBodyCodeCode {
    #[serde(rename = "BILLING_PROMOTION_CODE_INVALID")]
    BillingPromotionCodeInvalid,
}
impl fmt::Display for UnprocessableEntityErrorBodyCodeCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::BillingPromotionCodeInvalid => "BILLING_PROMOTION_CODE_INVALID",
        };
        write!(f, "{}", s)
    }
}
