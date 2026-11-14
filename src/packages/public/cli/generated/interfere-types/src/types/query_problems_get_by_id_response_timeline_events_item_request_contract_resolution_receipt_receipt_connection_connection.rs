pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionConnection {
        QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionConnectionConnectionId(QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionConnectionConnectionId),

        QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionConnectionIntegrationId(QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionConnectionIntegrationId),
}

impl QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionConnection {
    pub fn is_query_problems_get_by_id_response_timeline_events_item_request_contract_resolution_receipt_receipt_connection_connection_connection_id(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionConnectionConnectionId(_))
    }

    pub fn is_query_problems_get_by_id_response_timeline_events_item_request_contract_resolution_receipt_receipt_connection_connection_integration_id(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionConnectionIntegrationId(_))
    }


    pub fn as_query_problems_get_by_id_response_timeline_events_item_request_contract_resolution_receipt_receipt_connection_connection_connection_id(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionConnectionConnectionId> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionConnectionConnectionId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_request_contract_resolution_receipt_receipt_connection_connection_connection_id(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionConnectionConnectionId> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionConnectionConnectionId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_get_by_id_response_timeline_events_item_request_contract_resolution_receipt_receipt_connection_connection_integration_id(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionConnectionIntegrationId> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionConnectionIntegrationId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_request_contract_resolution_receipt_receipt_connection_connection_integration_id(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionConnectionIntegrationId> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionConnectionIntegrationId(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionConnection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionConnectionConnectionId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionConnectionIntegrationId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
