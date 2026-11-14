pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum UnprocessableEntityErrorBodyZeroCode {
    #[serde(rename = "INTEGRATION_ALREADY_CONNECTED")]
    IntegrationAlreadyConnected,
}
impl fmt::Display for UnprocessableEntityErrorBodyZeroCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::IntegrationAlreadyConnected => "INTEGRATION_ALREADY_CONNECTED",
        };
        write!(f, "{}", s)
    }
}
