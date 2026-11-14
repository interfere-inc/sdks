pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRequiredEvidenceKind {
        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRequiredEvidenceKindZero(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRequiredEvidenceKindZero),

        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRequiredEvidenceKindOne(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRequiredEvidenceKindOne),
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRequiredEvidenceKind {
    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_action_kind_required_evidence_kind_zero(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRequiredEvidenceKindZero(_))
    }

    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_action_kind_required_evidence_kind_one(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRequiredEvidenceKindOne(_))
    }


    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_content_action_kind_required_evidence_kind_zero(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRequiredEvidenceKindZero> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRequiredEvidenceKindZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_content_action_kind_required_evidence_kind_zero(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRequiredEvidenceKindZero> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRequiredEvidenceKindZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_content_action_kind_required_evidence_kind_one(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRequiredEvidenceKindOne> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRequiredEvidenceKindOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_content_action_kind_required_evidence_kind_one(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRequiredEvidenceKindOne> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRequiredEvidenceKindOne(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRequiredEvidenceKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRequiredEvidenceKindZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRequiredEvidenceKindOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
