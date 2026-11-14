pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum BadRequestErrorBodyCodeCode {
    #[serde(rename = "SUPPORT_INVALID_REQUEST")]
    SupportInvalidRequest,
}
impl fmt::Display for BadRequestErrorBodyCodeCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::SupportInvalidRequest => "SUPPORT_INVALID_REQUEST",
        };
        write!(f, "{}", s)
    }
}
