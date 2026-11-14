pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenance {
        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenanceConnectionId(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenanceConnectionId),

        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenanceIntegrationId(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenanceIntegrationId),
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenance {
    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_action_kind_mcp_setup_withdrawal_receipt_provenance_connection_id(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenanceConnectionId(_))
    }

    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_action_kind_mcp_setup_withdrawal_receipt_provenance_integration_id(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenanceIntegrationId(_))
    }


    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_content_action_kind_mcp_setup_withdrawal_receipt_provenance_connection_id(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenanceConnectionId> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenanceConnectionId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_content_action_kind_mcp_setup_withdrawal_receipt_provenance_connection_id(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenanceConnectionId> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenanceConnectionId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_content_action_kind_mcp_setup_withdrawal_receipt_provenance_integration_id(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenanceIntegrationId> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenanceIntegrationId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_content_action_kind_mcp_setup_withdrawal_receipt_provenance_integration_id(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenanceIntegrationId> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenanceIntegrationId(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenanceConnectionId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenanceIntegrationId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
