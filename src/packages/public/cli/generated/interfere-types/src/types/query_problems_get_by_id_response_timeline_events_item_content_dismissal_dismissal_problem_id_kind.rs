pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalDismissalProblemIdKind {
    #[serde(rename = "duplicateOf")]
    DuplicateOf,
}
impl fmt::Display for QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalDismissalProblemIdKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::DuplicateOf => "duplicateOf",
        };
        write!(f, "{}", s)
    }
}
