pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum QueryProblemsGetByIdResponseTimelineEventsItemRequestContractRequiredEvidenceKind {
        QueryProblemsGetByIdResponseTimelineEventsItemRequestContractRequiredEvidenceKindZero(QueryProblemsGetByIdResponseTimelineEventsItemRequestContractRequiredEvidenceKindZero),

        QueryProblemsGetByIdResponseTimelineEventsItemRequestContractRequiredEvidenceKindOne(QueryProblemsGetByIdResponseTimelineEventsItemRequestContractRequiredEvidenceKindOne),
}

impl QueryProblemsGetByIdResponseTimelineEventsItemRequestContractRequiredEvidenceKind {
    pub fn is_query_problems_get_by_id_response_timeline_events_item_request_contract_required_evidence_kind_zero(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemRequestContractRequiredEvidenceKindZero(_))
    }

    pub fn is_query_problems_get_by_id_response_timeline_events_item_request_contract_required_evidence_kind_one(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemRequestContractRequiredEvidenceKindOne(_))
    }


    pub fn as_query_problems_get_by_id_response_timeline_events_item_request_contract_required_evidence_kind_zero(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemRequestContractRequiredEvidenceKindZero> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemRequestContractRequiredEvidenceKindZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_request_contract_required_evidence_kind_zero(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractRequiredEvidenceKindZero> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemRequestContractRequiredEvidenceKindZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_get_by_id_response_timeline_events_item_request_contract_required_evidence_kind_one(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemRequestContractRequiredEvidenceKindOne> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemRequestContractRequiredEvidenceKindOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_request_contract_required_evidence_kind_one(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractRequiredEvidenceKindOne> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemRequestContractRequiredEvidenceKindOne(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryProblemsGetByIdResponseTimelineEventsItemRequestContractRequiredEvidenceKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryProblemsGetByIdResponseTimelineEventsItemRequestContractRequiredEvidenceKindZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsGetByIdResponseTimelineEventsItemRequestContractRequiredEvidenceKindOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
