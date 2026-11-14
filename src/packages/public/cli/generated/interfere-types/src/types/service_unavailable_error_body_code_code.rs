pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ServiceUnavailableErrorBodyCodeCode {
    #[serde(rename = "NETWORK_ACCESS_UNAVAILABLE")]
    NetworkAccessUnavailable,
}
impl fmt::Display for ServiceUnavailableErrorBodyCodeCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::NetworkAccessUnavailable => "NETWORK_ACCESS_UNAVAILABLE",
        };
        write!(f, "{}", s)
    }
}
