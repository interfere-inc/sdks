pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceipt {
        QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptZero(QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptZero),

        QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnection(QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnection),

        QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKind(QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKind),
}

impl QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceipt {
    pub fn is_query_problems_get_by_id_response_timeline_events_item_request_contract_resolution_receipt_receipt_zero(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptZero(_))
    }

    pub fn is_query_problems_get_by_id_response_timeline_events_item_request_contract_resolution_receipt_receipt_connection(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnection(_))
    }

    pub fn is_query_problems_get_by_id_response_timeline_events_item_request_contract_resolution_receipt_receipt_kind(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKind(_))
    }


    pub fn as_query_problems_get_by_id_response_timeline_events_item_request_contract_resolution_receipt_receipt_zero(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptZero> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_request_contract_resolution_receipt_receipt_zero(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptZero> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_get_by_id_response_timeline_events_item_request_contract_resolution_receipt_receipt_connection(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnection> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnection(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_request_contract_resolution_receipt_receipt_connection(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnection> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnection(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_get_by_id_response_timeline_events_item_request_contract_resolution_receipt_receipt_kind(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKind> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKind(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_request_contract_resolution_receipt_receipt_kind(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKind> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKind(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceipt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnection(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKind(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
