pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolution {
        QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionZero(QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionZero),

        QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceipt(QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceipt),

        QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionTwo(QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionTwo),

        QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionAt(QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionAt),
}

impl QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolution {
    pub fn is_query_problems_get_by_id_response_timeline_events_item_request_contract_resolution_zero(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionZero(_))
    }

    pub fn is_query_problems_get_by_id_response_timeline_events_item_request_contract_resolution_receipt(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceipt(_))
    }

    pub fn is_query_problems_get_by_id_response_timeline_events_item_request_contract_resolution_two(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionTwo(_))
    }

    pub fn is_query_problems_get_by_id_response_timeline_events_item_request_contract_resolution_at(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionAt(_))
    }


    pub fn as_query_problems_get_by_id_response_timeline_events_item_request_contract_resolution_zero(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionZero> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_request_contract_resolution_zero(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionZero> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_get_by_id_response_timeline_events_item_request_contract_resolution_receipt(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceipt> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceipt(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_request_contract_resolution_receipt(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceipt> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceipt(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_get_by_id_response_timeline_events_item_request_contract_resolution_two(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionTwo> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionTwo(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_request_contract_resolution_two(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionTwo> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionTwo(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_get_by_id_response_timeline_events_item_request_contract_resolution_at(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionAt> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionAt(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_request_contract_resolution_at(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionAt> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionAt(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceipt(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionTwo(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionAt(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
