pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocContentItem {
        QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocContentItemZero(QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocContentItemZero),

        QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocContentItemOne(QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocContentItemOne),

        QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocContentItemType(QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocContentItemType),
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocContentItem {
    pub fn is_query_problems_get_by_id_response_timeline_events_item_content_evidence_gap_doc_content_item_zero(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocContentItemZero(_))
    }

    pub fn is_query_problems_get_by_id_response_timeline_events_item_content_evidence_gap_doc_content_item_one(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocContentItemOne(_))
    }

    pub fn is_query_problems_get_by_id_response_timeline_events_item_content_evidence_gap_doc_content_item_type(&self) -> bool {
        matches!(self, Self::QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocContentItemType(_))
    }


    pub fn as_query_problems_get_by_id_response_timeline_events_item_content_evidence_gap_doc_content_item_zero(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocContentItemZero> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocContentItemZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_content_evidence_gap_doc_content_item_zero(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocContentItemZero> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocContentItemZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_get_by_id_response_timeline_events_item_content_evidence_gap_doc_content_item_one(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocContentItemOne> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocContentItemOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_content_evidence_gap_doc_content_item_one(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocContentItemOne> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocContentItemOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_get_by_id_response_timeline_events_item_content_evidence_gap_doc_content_item_type(&self) -> Option<&QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocContentItemType> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocContentItemType(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_get_by_id_response_timeline_events_item_content_evidence_gap_doc_content_item_type(self) -> Option<QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocContentItemType> {
        match self {
                    Self::QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocContentItemType(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocContentItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocContentItemZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocContentItemOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDocContentItemType(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
