pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum TimelineEventContentCorrectionDocContentItem {
        TimelineEventContentCorrectionDocContentItemZero(TimelineEventContentCorrectionDocContentItemZero),

        TimelineEventContentCorrectionDocContentItemOne(TimelineEventContentCorrectionDocContentItemOne),

        TimelineEventContentCorrectionDocContentItemType(TimelineEventContentCorrectionDocContentItemType),
}

impl TimelineEventContentCorrectionDocContentItem {
    pub fn is_timeline_event_content_correction_doc_content_item_zero(&self) -> bool {
        matches!(self, Self::TimelineEventContentCorrectionDocContentItemZero(_))
    }

    pub fn is_timeline_event_content_correction_doc_content_item_one(&self) -> bool {
        matches!(self, Self::TimelineEventContentCorrectionDocContentItemOne(_))
    }

    pub fn is_timeline_event_content_correction_doc_content_item_type(&self) -> bool {
        matches!(self, Self::TimelineEventContentCorrectionDocContentItemType(_))
    }


    pub fn as_timeline_event_content_correction_doc_content_item_zero(&self) -> Option<&TimelineEventContentCorrectionDocContentItemZero> {
        match self {
                    Self::TimelineEventContentCorrectionDocContentItemZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_correction_doc_content_item_zero(self) -> Option<TimelineEventContentCorrectionDocContentItemZero> {
        match self {
                    Self::TimelineEventContentCorrectionDocContentItemZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_correction_doc_content_item_one(&self) -> Option<&TimelineEventContentCorrectionDocContentItemOne> {
        match self {
                    Self::TimelineEventContentCorrectionDocContentItemOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_correction_doc_content_item_one(self) -> Option<TimelineEventContentCorrectionDocContentItemOne> {
        match self {
                    Self::TimelineEventContentCorrectionDocContentItemOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_correction_doc_content_item_type(&self) -> Option<&TimelineEventContentCorrectionDocContentItemType> {
        match self {
                    Self::TimelineEventContentCorrectionDocContentItemType(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_correction_doc_content_item_type(self) -> Option<TimelineEventContentCorrectionDocContentItemType> {
        match self {
                    Self::TimelineEventContentCorrectionDocContentItemType(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for TimelineEventContentCorrectionDocContentItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TimelineEventContentCorrectionDocContentItemZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentCorrectionDocContentItemOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentCorrectionDocContentItemType(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
