pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum TimelineEventRequestContractResolutionReceiptReceipt {
        TimelineEventRequestContractResolutionReceiptReceiptZero(TimelineEventRequestContractResolutionReceiptReceiptZero),

        TimelineEventRequestContractResolutionReceiptReceiptConnection(TimelineEventRequestContractResolutionReceiptReceiptConnection),

        TimelineEventRequestContractResolutionReceiptReceiptKind(TimelineEventRequestContractResolutionReceiptReceiptKind),
}

impl TimelineEventRequestContractResolutionReceiptReceipt {
    pub fn is_timeline_event_request_contract_resolution_receipt_receipt_zero(&self) -> bool {
        matches!(self, Self::TimelineEventRequestContractResolutionReceiptReceiptZero(_))
    }

    pub fn is_timeline_event_request_contract_resolution_receipt_receipt_connection(&self) -> bool {
        matches!(self, Self::TimelineEventRequestContractResolutionReceiptReceiptConnection(_))
    }

    pub fn is_timeline_event_request_contract_resolution_receipt_receipt_kind(&self) -> bool {
        matches!(self, Self::TimelineEventRequestContractResolutionReceiptReceiptKind(_))
    }


    pub fn as_timeline_event_request_contract_resolution_receipt_receipt_zero(&self) -> Option<&TimelineEventRequestContractResolutionReceiptReceiptZero> {
        match self {
                    Self::TimelineEventRequestContractResolutionReceiptReceiptZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_request_contract_resolution_receipt_receipt_zero(self) -> Option<TimelineEventRequestContractResolutionReceiptReceiptZero> {
        match self {
                    Self::TimelineEventRequestContractResolutionReceiptReceiptZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_request_contract_resolution_receipt_receipt_connection(&self) -> Option<&TimelineEventRequestContractResolutionReceiptReceiptConnection> {
        match self {
                    Self::TimelineEventRequestContractResolutionReceiptReceiptConnection(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_request_contract_resolution_receipt_receipt_connection(self) -> Option<TimelineEventRequestContractResolutionReceiptReceiptConnection> {
        match self {
                    Self::TimelineEventRequestContractResolutionReceiptReceiptConnection(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_request_contract_resolution_receipt_receipt_kind(&self) -> Option<&TimelineEventRequestContractResolutionReceiptReceiptKind> {
        match self {
                    Self::TimelineEventRequestContractResolutionReceiptReceiptKind(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_request_contract_resolution_receipt_receipt_kind(self) -> Option<TimelineEventRequestContractResolutionReceiptReceiptKind> {
        match self {
                    Self::TimelineEventRequestContractResolutionReceiptReceiptKind(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for TimelineEventRequestContractResolutionReceiptReceipt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TimelineEventRequestContractResolutionReceiptReceiptZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventRequestContractResolutionReceiptReceiptConnection(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventRequestContractResolutionReceiptReceiptKind(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
