pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum NotFoundErrorBodyTwoCode {
    #[serde(rename = "SOURCE_INTEGRATION_UNSUPPORTED")]
    SourceIntegrationUnsupported,
}
impl fmt::Display for NotFoundErrorBodyTwoCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::SourceIntegrationUnsupported => "SOURCE_INTEGRATION_UNSUPPORTED",
        };
        write!(f, "{}", s)
    }
}
