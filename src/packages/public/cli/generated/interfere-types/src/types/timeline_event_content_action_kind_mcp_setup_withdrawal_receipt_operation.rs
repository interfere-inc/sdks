pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperation {
        TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationCursor(TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationCursor),

        TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationArguments(TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationArguments),

        TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationKind(TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationKind),
}

impl TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperation {
    pub fn is_timeline_event_content_action_kind_mcp_setup_withdrawal_receipt_operation_cursor(&self) -> bool {
        matches!(self, Self::TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationCursor(_))
    }

    pub fn is_timeline_event_content_action_kind_mcp_setup_withdrawal_receipt_operation_arguments(&self) -> bool {
        matches!(self, Self::TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationArguments(_))
    }

    pub fn is_timeline_event_content_action_kind_mcp_setup_withdrawal_receipt_operation_kind(&self) -> bool {
        matches!(self, Self::TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationKind(_))
    }


    pub fn as_timeline_event_content_action_kind_mcp_setup_withdrawal_receipt_operation_cursor(&self) -> Option<&TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationCursor> {
        match self {
                    Self::TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationCursor(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_action_kind_mcp_setup_withdrawal_receipt_operation_cursor(self) -> Option<TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationCursor> {
        match self {
                    Self::TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationCursor(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_action_kind_mcp_setup_withdrawal_receipt_operation_arguments(&self) -> Option<&TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationArguments> {
        match self {
                    Self::TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationArguments(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_action_kind_mcp_setup_withdrawal_receipt_operation_arguments(self) -> Option<TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationArguments> {
        match self {
                    Self::TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationArguments(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_action_kind_mcp_setup_withdrawal_receipt_operation_kind(&self) -> Option<&TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationKind> {
        match self {
                    Self::TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationKind(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_action_kind_mcp_setup_withdrawal_receipt_operation_kind(self) -> Option<TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationKind> {
        match self {
                    Self::TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationKind(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationCursor(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationArguments(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationKind(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
