pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublication {
        QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationBasis(QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationBasis),

        QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationReason(QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationReason),
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublication {
    pub fn is_query_problems_get_by_id_response_timeline_events_item_content_evidence_gap_publication_basis(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationBasis(_))
    }

    pub fn is_query_problems_get_by_id_response_timeline_events_item_content_evidence_gap_publication_reason(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationReason(_))
    }


    pub fn as_query_problems_get_by_id_response_timeline_events_item_content_evidence_gap_publication_basis(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationBasis> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationBasis(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_content_evidence_gap_publication_basis(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationBasis> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationBasis(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_get_by_id_response_timeline_events_item_content_evidence_gap_publication_reason(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationReason> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationReason(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_content_evidence_gap_publication_reason(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationReason> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationReason(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublication {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationBasis(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationReason(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
