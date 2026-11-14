pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum UnprocessableEntityErrorBodyOneCode {
    #[serde(rename = "INTEGRATION_HOST_INVALID")]
    IntegrationHostInvalid,
}
impl fmt::Display for UnprocessableEntityErrorBodyOneCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::IntegrationHostInvalid => "INTEGRATION_HOST_INVALID",
        };
        write!(f, "{}", s)
    }
}
