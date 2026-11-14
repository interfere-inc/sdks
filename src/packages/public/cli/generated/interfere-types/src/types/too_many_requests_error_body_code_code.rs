pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TooManyRequestsErrorBodyCodeCode {
    #[serde(rename = "KEY_VALIDATION_RATE_LIMITED")]
    KeyValidationRateLimited,
}
impl fmt::Display for TooManyRequestsErrorBodyCodeCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::KeyValidationRateLimited => "KEY_VALIDATION_RATE_LIMITED",
        };
        write!(f, "{}", s)
    }
}
