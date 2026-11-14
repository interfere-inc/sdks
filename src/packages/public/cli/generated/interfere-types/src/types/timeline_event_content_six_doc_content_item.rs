pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum TimelineEventContentSixDocContentItem {
        TimelineEventContentSixDocContentItemZero(TimelineEventContentSixDocContentItemZero),

        TimelineEventContentSixDocContentItemOne(TimelineEventContentSixDocContentItemOne),

        TimelineEventContentSixDocContentItemType(TimelineEventContentSixDocContentItemType),
}

impl TimelineEventContentSixDocContentItem {
    pub fn is_timeline_event_content_six_doc_content_item_zero(&self) -> bool {
        matches!(self, Self::TimelineEventContentSixDocContentItemZero(_))
    }

    pub fn is_timeline_event_content_six_doc_content_item_one(&self) -> bool {
        matches!(self, Self::TimelineEventContentSixDocContentItemOne(_))
    }

    pub fn is_timeline_event_content_six_doc_content_item_type(&self) -> bool {
        matches!(self, Self::TimelineEventContentSixDocContentItemType(_))
    }


    pub fn as_timeline_event_content_six_doc_content_item_zero(&self) -> Option<&TimelineEventContentSixDocContentItemZero> {
        match self {
                    Self::TimelineEventContentSixDocContentItemZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_six_doc_content_item_zero(self) -> Option<TimelineEventContentSixDocContentItemZero> {
        match self {
                    Self::TimelineEventContentSixDocContentItemZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_six_doc_content_item_one(&self) -> Option<&TimelineEventContentSixDocContentItemOne> {
        match self {
                    Self::TimelineEventContentSixDocContentItemOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_six_doc_content_item_one(self) -> Option<TimelineEventContentSixDocContentItemOne> {
        match self {
                    Self::TimelineEventContentSixDocContentItemOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_six_doc_content_item_type(&self) -> Option<&TimelineEventContentSixDocContentItemType> {
        match self {
                    Self::TimelineEventContentSixDocContentItemType(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_six_doc_content_item_type(self) -> Option<TimelineEventContentSixDocContentItemType> {
        match self {
                    Self::TimelineEventContentSixDocContentItemType(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for TimelineEventContentSixDocContentItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TimelineEventContentSixDocContentItemZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentSixDocContentItemOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentSixDocContentItemType(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
