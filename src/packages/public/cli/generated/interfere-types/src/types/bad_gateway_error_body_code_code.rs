pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum BadGatewayErrorBodyCodeCode {
    #[serde(rename = "FEEDBACK_UNAVAILABLE")]
    FeedbackUnavailable,
}
impl fmt::Display for BadGatewayErrorBodyCodeCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::FeedbackUnavailable => "FEEDBACK_UNAVAILABLE",
        };
        write!(f, "{}", s)
    }
}
