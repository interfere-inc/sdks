pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptOperationArgumentsKind {
    #[serde(rename = "tool")]
    Tool,
}
impl fmt::Display for QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptOperationArgumentsKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Tool => "tool",
        };
        write!(f, "{}", s)
    }
}
