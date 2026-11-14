pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletion {
        QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionActionKind(QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionActionKind),

        QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionConcern(QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionConcern),

        QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionIntegrationName(QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionIntegrationName),

        QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionGapKind(QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionGapKind),
}

impl QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletion {
    pub fn is_query_problems_get_by_id_response_timeline_events_item_request_contract_completion_action_kind(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionActionKind(_))
    }

    pub fn is_query_problems_get_by_id_response_timeline_events_item_request_contract_completion_concern(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionConcern(_))
    }

    pub fn is_query_problems_get_by_id_response_timeline_events_item_request_contract_completion_integration_name(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionIntegrationName(_))
    }

    pub fn is_query_problems_get_by_id_response_timeline_events_item_request_contract_completion_gap_kind(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionGapKind(_))
    }


    pub fn as_query_problems_get_by_id_response_timeline_events_item_request_contract_completion_action_kind(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionActionKind> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionActionKind(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_request_contract_completion_action_kind(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionActionKind> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionActionKind(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_get_by_id_response_timeline_events_item_request_contract_completion_concern(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionConcern> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionConcern(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_request_contract_completion_concern(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionConcern> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionConcern(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_get_by_id_response_timeline_events_item_request_contract_completion_integration_name(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionIntegrationName> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionIntegrationName(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_request_contract_completion_integration_name(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionIntegrationName> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionIntegrationName(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_get_by_id_response_timeline_events_item_request_contract_completion_gap_kind(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionGapKind> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionGapKind(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_request_contract_completion_gap_kind(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionGapKind> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionGapKind(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionActionKind(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionConcern(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionIntegrationName(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionGapKind(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
