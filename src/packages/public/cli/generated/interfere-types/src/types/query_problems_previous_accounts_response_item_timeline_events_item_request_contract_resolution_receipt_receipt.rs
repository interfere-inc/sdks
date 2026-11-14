pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceipt {
        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptZero(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptZero),

        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptConnection(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptConnection),

        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptKind(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptKind),
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceipt {
    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_request_contract_resolution_receipt_receipt_zero(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptZero(_))
    }

    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_request_contract_resolution_receipt_receipt_connection(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptConnection(_))
    }

    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_request_contract_resolution_receipt_receipt_kind(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptKind(_))
    }


    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_request_contract_resolution_receipt_receipt_zero(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptZero> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_request_contract_resolution_receipt_receipt_zero(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptZero> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_request_contract_resolution_receipt_receipt_connection(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptConnection> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptConnection(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_request_contract_resolution_receipt_receipt_connection(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptConnection> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptConnection(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_request_contract_resolution_receipt_receipt_kind(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptKind> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptKind(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_request_contract_resolution_receipt_receipt_kind(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptKind> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptKind(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceipt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptConnection(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptKind(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
