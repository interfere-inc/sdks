pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ForbiddenErrorBodyZeroCode {
    #[serde(rename = "NETWORK_ACCESS_DENIED")]
    NetworkAccessDenied,
}
impl fmt::Display for ForbiddenErrorBodyZeroCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::NetworkAccessDenied => "NETWORK_ACCESS_DENIED",
        };
        write!(f, "{}", s)
    }
}
