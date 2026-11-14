pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractRequiredEvidenceKind {
        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractRequiredEvidenceKindZero(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractRequiredEvidenceKindZero),

        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractRequiredEvidenceKindOne(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractRequiredEvidenceKindOne),
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractRequiredEvidenceKind {
    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_request_contract_required_evidence_kind_zero(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractRequiredEvidenceKindZero(_))
    }

    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_request_contract_required_evidence_kind_one(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractRequiredEvidenceKindOne(_))
    }


    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_request_contract_required_evidence_kind_zero(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractRequiredEvidenceKindZero> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractRequiredEvidenceKindZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_request_contract_required_evidence_kind_zero(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractRequiredEvidenceKindZero> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractRequiredEvidenceKindZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_request_contract_required_evidence_kind_one(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractRequiredEvidenceKindOne> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractRequiredEvidenceKindOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_request_contract_required_evidence_kind_one(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractRequiredEvidenceKindOne> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractRequiredEvidenceKindOne(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractRequiredEvidenceKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractRequiredEvidenceKindZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractRequiredEvidenceKindOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
