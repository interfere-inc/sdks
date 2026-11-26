pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum UnprocessableEntityErrorBodyCodeCode {
    #[serde(rename = "PROBLEM_DUPLICATE_OF_ITSELF")]
    ProblemDuplicateOfItself,
}
impl fmt::Display for UnprocessableEntityErrorBodyCodeCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::ProblemDuplicateOfItself => "PROBLEM_DUPLICATE_OF_ITSELF",
        };
        write!(f, "{}", s)
    }
}
