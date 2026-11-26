pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ContentTooLargeErrorBodyCodeCode {
    #[serde(rename = "PROBLEM_CORRECTION_TOO_LARGE")]
    ProblemCorrectionTooLarge,
}
impl fmt::Display for ContentTooLargeErrorBodyCodeCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::ProblemCorrectionTooLarge => "PROBLEM_CORRECTION_TOO_LARGE",
        };
        write!(f, "{}", s)
    }
}
