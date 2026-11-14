pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptOperation {
        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptOperationCursor(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptOperationCursor),

        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptOperationArguments(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptOperationArguments),

        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptOperationKind(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptOperationKind),
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptOperation {
    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_action_kind_mcp_setup_withdrawal_receipt_operation_cursor(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptOperationCursor(_))
    }

    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_action_kind_mcp_setup_withdrawal_receipt_operation_arguments(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptOperationArguments(_))
    }

    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_action_kind_mcp_setup_withdrawal_receipt_operation_kind(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptOperationKind(_))
    }


    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_content_action_kind_mcp_setup_withdrawal_receipt_operation_cursor(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptOperationCursor> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptOperationCursor(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_content_action_kind_mcp_setup_withdrawal_receipt_operation_cursor(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptOperationCursor> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptOperationCursor(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_content_action_kind_mcp_setup_withdrawal_receipt_operation_arguments(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptOperationArguments> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptOperationArguments(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_content_action_kind_mcp_setup_withdrawal_receipt_operation_arguments(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptOperationArguments> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptOperationArguments(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_content_action_kind_mcp_setup_withdrawal_receipt_operation_kind(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptOperationKind> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptOperationKind(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_content_action_kind_mcp_setup_withdrawal_receipt_operation_kind(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptOperationKind> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptOperationKind(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptOperationCursor(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptOperationArguments(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptOperationKind(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
