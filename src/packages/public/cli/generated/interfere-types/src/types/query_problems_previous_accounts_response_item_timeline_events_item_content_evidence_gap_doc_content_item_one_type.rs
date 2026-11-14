pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemOneType {
    #[serde(rename = "codeBlock")]
    CodeBlock,
}
impl fmt::Display for QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemOneType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::CodeBlock => "codeBlock",
        };
        write!(f, "{}", s)
    }
}
