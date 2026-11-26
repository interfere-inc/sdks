pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseDocContentItemOneType {
    #[serde(rename = "codeBlock")]
    CodeBlock,
}
impl fmt::Display for QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseDocContentItemOneType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::CodeBlock => "codeBlock",
        };
        write!(f, "{}", s)
    }
}
