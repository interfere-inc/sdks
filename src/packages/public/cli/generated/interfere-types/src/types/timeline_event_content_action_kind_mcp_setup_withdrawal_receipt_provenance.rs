pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum TimelineEventContentActionKindMcpSetupWithdrawalReceiptProvenance {
        TimelineEventContentActionKindMcpSetupWithdrawalReceiptProvenanceConnectionId(TimelineEventContentActionKindMcpSetupWithdrawalReceiptProvenanceConnectionId),

        TimelineEventContentActionKindMcpSetupWithdrawalReceiptProvenanceIntegrationId(TimelineEventContentActionKindMcpSetupWithdrawalReceiptProvenanceIntegrationId),
}

impl TimelineEventContentActionKindMcpSetupWithdrawalReceiptProvenance {
    pub fn is_timeline_event_content_action_kind_mcp_setup_withdrawal_receipt_provenance_connection_id(&self) -> bool {
        matches!(self, Self::TimelineEventContentActionKindMcpSetupWithdrawalReceiptProvenanceConnectionId(_))
    }

    pub fn is_timeline_event_content_action_kind_mcp_setup_withdrawal_receipt_provenance_integration_id(&self) -> bool {
        matches!(self, Self::TimelineEventContentActionKindMcpSetupWithdrawalReceiptProvenanceIntegrationId(_))
    }


    pub fn as_timeline_event_content_action_kind_mcp_setup_withdrawal_receipt_provenance_connection_id(&self) -> Option<&TimelineEventContentActionKindMcpSetupWithdrawalReceiptProvenanceConnectionId> {
        match self {
                    Self::TimelineEventContentActionKindMcpSetupWithdrawalReceiptProvenanceConnectionId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_action_kind_mcp_setup_withdrawal_receipt_provenance_connection_id(self) -> Option<TimelineEventContentActionKindMcpSetupWithdrawalReceiptProvenanceConnectionId> {
        match self {
                    Self::TimelineEventContentActionKindMcpSetupWithdrawalReceiptProvenanceConnectionId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_action_kind_mcp_setup_withdrawal_receipt_provenance_integration_id(&self) -> Option<&TimelineEventContentActionKindMcpSetupWithdrawalReceiptProvenanceIntegrationId> {
        match self {
                    Self::TimelineEventContentActionKindMcpSetupWithdrawalReceiptProvenanceIntegrationId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_action_kind_mcp_setup_withdrawal_receipt_provenance_integration_id(self) -> Option<TimelineEventContentActionKindMcpSetupWithdrawalReceiptProvenanceIntegrationId> {
        match self {
                    Self::TimelineEventContentActionKindMcpSetupWithdrawalReceiptProvenanceIntegrationId(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for TimelineEventContentActionKindMcpSetupWithdrawalReceiptProvenance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TimelineEventContentActionKindMcpSetupWithdrawalReceiptProvenanceConnectionId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentActionKindMcpSetupWithdrawalReceiptProvenanceIntegrationId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
