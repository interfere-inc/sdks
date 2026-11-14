pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRequiredEvidenceKind {
        QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRequiredEvidenceKindZero(QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRequiredEvidenceKindZero),

        QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRequiredEvidenceKindOne(QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRequiredEvidenceKindOne),
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRequiredEvidenceKind {
    pub fn is_query_problems_get_by_id_response_timeline_events_item_content_action_kind_required_evidence_kind_zero(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRequiredEvidenceKindZero(_))
    }

    pub fn is_query_problems_get_by_id_response_timeline_events_item_content_action_kind_required_evidence_kind_one(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRequiredEvidenceKindOne(_))
    }


    pub fn as_query_problems_get_by_id_response_timeline_events_item_content_action_kind_required_evidence_kind_zero(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRequiredEvidenceKindZero> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRequiredEvidenceKindZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_content_action_kind_required_evidence_kind_zero(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRequiredEvidenceKindZero> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRequiredEvidenceKindZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_get_by_id_response_timeline_events_item_content_action_kind_required_evidence_kind_one(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRequiredEvidenceKindOne> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRequiredEvidenceKindOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_content_action_kind_required_evidence_kind_one(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRequiredEvidenceKindOne> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRequiredEvidenceKindOne(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRequiredEvidenceKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRequiredEvidenceKindZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRequiredEvidenceKindOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
