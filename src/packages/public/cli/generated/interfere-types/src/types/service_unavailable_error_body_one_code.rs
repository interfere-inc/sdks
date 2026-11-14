pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ServiceUnavailableErrorBodyOneCode {
    #[serde(rename = "OPERATION_UNAVAILABLE")]
    OperationUnavailable,
}
impl fmt::Display for ServiceUnavailableErrorBodyOneCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::OperationUnavailable => "OPERATION_UNAVAILABLE",
        };
        write!(f, "{}", s)
    }
}
