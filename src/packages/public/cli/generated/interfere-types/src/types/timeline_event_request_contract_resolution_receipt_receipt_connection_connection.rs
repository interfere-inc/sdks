pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum TimelineEventRequestContractResolutionReceiptReceiptConnectionConnection {
        TimelineEventRequestContractResolutionReceiptReceiptConnectionConnectionConnectionId(TimelineEventRequestContractResolutionReceiptReceiptConnectionConnectionConnectionId),

        TimelineEventRequestContractResolutionReceiptReceiptConnectionConnectionIntegrationId(TimelineEventRequestContractResolutionReceiptReceiptConnectionConnectionIntegrationId),
}

impl TimelineEventRequestContractResolutionReceiptReceiptConnectionConnection {
    pub fn is_timeline_event_request_contract_resolution_receipt_receipt_connection_connection_connection_id(&self) -> bool {
        matches!(self, Self::TimelineEventRequestContractResolutionReceiptReceiptConnectionConnectionConnectionId(_))
    }

    pub fn is_timeline_event_request_contract_resolution_receipt_receipt_connection_connection_integration_id(&self) -> bool {
        matches!(self, Self::TimelineEventRequestContractResolutionReceiptReceiptConnectionConnectionIntegrationId(_))
    }


    pub fn as_timeline_event_request_contract_resolution_receipt_receipt_connection_connection_connection_id(&self) -> Option<&TimelineEventRequestContractResolutionReceiptReceiptConnectionConnectionConnectionId> {
        match self {
                    Self::TimelineEventRequestContractResolutionReceiptReceiptConnectionConnectionConnectionId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_request_contract_resolution_receipt_receipt_connection_connection_connection_id(self) -> Option<TimelineEventRequestContractResolutionReceiptReceiptConnectionConnectionConnectionId> {
        match self {
                    Self::TimelineEventRequestContractResolutionReceiptReceiptConnectionConnectionConnectionId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_request_contract_resolution_receipt_receipt_connection_connection_integration_id(&self) -> Option<&TimelineEventRequestContractResolutionReceiptReceiptConnectionConnectionIntegrationId> {
        match self {
                    Self::TimelineEventRequestContractResolutionReceiptReceiptConnectionConnectionIntegrationId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_request_contract_resolution_receipt_receipt_connection_connection_integration_id(self) -> Option<TimelineEventRequestContractResolutionReceiptReceiptConnectionConnectionIntegrationId> {
        match self {
                    Self::TimelineEventRequestContractResolutionReceiptReceiptConnectionConnectionIntegrationId(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for TimelineEventRequestContractResolutionReceiptReceiptConnectionConnection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TimelineEventRequestContractResolutionReceiptReceiptConnectionConnectionConnectionId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventRequestContractResolutionReceiptReceiptConnectionConnectionIntegrationId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
