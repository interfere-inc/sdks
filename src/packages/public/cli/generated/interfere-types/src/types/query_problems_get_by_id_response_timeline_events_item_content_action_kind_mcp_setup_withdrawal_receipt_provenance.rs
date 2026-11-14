pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenance {
        QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenanceConnectionId(QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenanceConnectionId),

        QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenanceIntegrationId(QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenanceIntegrationId),
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenance {
    pub fn is_query_problems_get_by_id_response_timeline_events_item_content_action_kind_mcp_setup_withdrawal_receipt_provenance_connection_id(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenanceConnectionId(_))
    }

    pub fn is_query_problems_get_by_id_response_timeline_events_item_content_action_kind_mcp_setup_withdrawal_receipt_provenance_integration_id(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenanceIntegrationId(_))
    }


    pub fn as_query_problems_get_by_id_response_timeline_events_item_content_action_kind_mcp_setup_withdrawal_receipt_provenance_connection_id(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenanceConnectionId> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenanceConnectionId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_content_action_kind_mcp_setup_withdrawal_receipt_provenance_connection_id(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenanceConnectionId> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenanceConnectionId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_get_by_id_response_timeline_events_item_content_action_kind_mcp_setup_withdrawal_receipt_provenance_integration_id(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenanceIntegrationId> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenanceIntegrationId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_content_action_kind_mcp_setup_withdrawal_receipt_provenance_integration_id(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenanceIntegrationId> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenanceIntegrationId(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenanceConnectionId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenanceIntegrationId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
