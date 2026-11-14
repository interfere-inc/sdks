pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolution {
        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionZero(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionZero),

        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceipt(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceipt),

        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionTwo(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionTwo),

        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionAt(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionAt),
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolution {
    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_request_contract_resolution_zero(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionZero(_))
    }

    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_request_contract_resolution_receipt(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceipt(_))
    }

    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_request_contract_resolution_two(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionTwo(_))
    }

    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_request_contract_resolution_at(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionAt(_))
    }


    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_request_contract_resolution_zero(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionZero> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_request_contract_resolution_zero(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionZero> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_request_contract_resolution_receipt(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceipt> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceipt(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_request_contract_resolution_receipt(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceipt> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceipt(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_request_contract_resolution_two(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionTwo> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionTwo(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_request_contract_resolution_two(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionTwo> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionTwo(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_request_contract_resolution_at(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionAt> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionAt(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_request_contract_resolution_at(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionAt> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionAt(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceipt(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionTwo(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionAt(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
