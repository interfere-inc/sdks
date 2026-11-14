pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum TimelineEventContentEvidenceGapDocContentItem {
        TimelineEventContentEvidenceGapDocContentItemZero(TimelineEventContentEvidenceGapDocContentItemZero),

        TimelineEventContentEvidenceGapDocContentItemOne(TimelineEventContentEvidenceGapDocContentItemOne),

        TimelineEventContentEvidenceGapDocContentItemType(TimelineEventContentEvidenceGapDocContentItemType),
}

impl TimelineEventContentEvidenceGapDocContentItem {
    pub fn is_timeline_event_content_evidence_gap_doc_content_item_zero(&self) -> bool {
        matches!(self, Self::TimelineEventContentEvidenceGapDocContentItemZero(_))
    }

    pub fn is_timeline_event_content_evidence_gap_doc_content_item_one(&self) -> bool {
        matches!(self, Self::TimelineEventContentEvidenceGapDocContentItemOne(_))
    }

    pub fn is_timeline_event_content_evidence_gap_doc_content_item_type(&self) -> bool {
        matches!(self, Self::TimelineEventContentEvidenceGapDocContentItemType(_))
    }


    pub fn as_timeline_event_content_evidence_gap_doc_content_item_zero(&self) -> Option<&TimelineEventContentEvidenceGapDocContentItemZero> {
        match self {
                    Self::TimelineEventContentEvidenceGapDocContentItemZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_evidence_gap_doc_content_item_zero(self) -> Option<TimelineEventContentEvidenceGapDocContentItemZero> {
        match self {
                    Self::TimelineEventContentEvidenceGapDocContentItemZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_evidence_gap_doc_content_item_one(&self) -> Option<&TimelineEventContentEvidenceGapDocContentItemOne> {
        match self {
                    Self::TimelineEventContentEvidenceGapDocContentItemOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_evidence_gap_doc_content_item_one(self) -> Option<TimelineEventContentEvidenceGapDocContentItemOne> {
        match self {
                    Self::TimelineEventContentEvidenceGapDocContentItemOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_evidence_gap_doc_content_item_type(&self) -> Option<&TimelineEventContentEvidenceGapDocContentItemType> {
        match self {
                    Self::TimelineEventContentEvidenceGapDocContentItemType(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_evidence_gap_doc_content_item_type(self) -> Option<TimelineEventContentEvidenceGapDocContentItemType> {
        match self {
                    Self::TimelineEventContentEvidenceGapDocContentItemType(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for TimelineEventContentEvidenceGapDocContentItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TimelineEventContentEvidenceGapDocContentItemZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentEvidenceGapDocContentItemOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentEvidenceGapDocContentItemType(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
