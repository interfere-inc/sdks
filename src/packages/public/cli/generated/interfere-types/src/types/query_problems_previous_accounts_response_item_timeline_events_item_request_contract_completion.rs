pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletion {
        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionActionKind(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionActionKind),

        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionConcern(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionConcern),

        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionIntegrationName(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionIntegrationName),

        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionGapKind(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionGapKind),
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletion {
    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_request_contract_completion_action_kind(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionActionKind(_))
    }

    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_request_contract_completion_concern(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionConcern(_))
    }

    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_request_contract_completion_integration_name(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionIntegrationName(_))
    }

    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_request_contract_completion_gap_kind(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionGapKind(_))
    }


    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_request_contract_completion_action_kind(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionActionKind> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionActionKind(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_request_contract_completion_action_kind(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionActionKind> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionActionKind(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_request_contract_completion_concern(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionConcern> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionConcern(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_request_contract_completion_concern(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionConcern> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionConcern(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_request_contract_completion_integration_name(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionIntegrationName> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionIntegrationName(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_request_contract_completion_integration_name(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionIntegrationName> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionIntegrationName(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_request_contract_completion_gap_kind(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionGapKind> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionGapKind(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_request_contract_completion_gap_kind(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionGapKind> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionGapKind(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionActionKind(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionConcern(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionIntegrationName(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionGapKind(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
