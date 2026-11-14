pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum TimelineEventContentElevenOneDocContentItem {
        TimelineEventContentElevenOneDocContentItemZero(TimelineEventContentElevenOneDocContentItemZero),

        TimelineEventContentElevenOneDocContentItemOne(TimelineEventContentElevenOneDocContentItemOne),

        TimelineEventContentElevenOneDocContentItemType(TimelineEventContentElevenOneDocContentItemType),
}

impl TimelineEventContentElevenOneDocContentItem {
    pub fn is_timeline_event_content_eleven_one_doc_content_item_zero(&self) -> bool {
        matches!(self, Self::TimelineEventContentElevenOneDocContentItemZero(_))
    }

    pub fn is_timeline_event_content_eleven_one_doc_content_item_one(&self) -> bool {
        matches!(self, Self::TimelineEventContentElevenOneDocContentItemOne(_))
    }

    pub fn is_timeline_event_content_eleven_one_doc_content_item_type(&self) -> bool {
        matches!(self, Self::TimelineEventContentElevenOneDocContentItemType(_))
    }


    pub fn as_timeline_event_content_eleven_one_doc_content_item_zero(&self) -> Option<&TimelineEventContentElevenOneDocContentItemZero> {
        match self {
                    Self::TimelineEventContentElevenOneDocContentItemZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_eleven_one_doc_content_item_zero(self) -> Option<TimelineEventContentElevenOneDocContentItemZero> {
        match self {
                    Self::TimelineEventContentElevenOneDocContentItemZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_eleven_one_doc_content_item_one(&self) -> Option<&TimelineEventContentElevenOneDocContentItemOne> {
        match self {
                    Self::TimelineEventContentElevenOneDocContentItemOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_eleven_one_doc_content_item_one(self) -> Option<TimelineEventContentElevenOneDocContentItemOne> {
        match self {
                    Self::TimelineEventContentElevenOneDocContentItemOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_eleven_one_doc_content_item_type(&self) -> Option<&TimelineEventContentElevenOneDocContentItemType> {
        match self {
                    Self::TimelineEventContentElevenOneDocContentItemType(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_eleven_one_doc_content_item_type(self) -> Option<TimelineEventContentElevenOneDocContentItemType> {
        match self {
                    Self::TimelineEventContentElevenOneDocContentItemType(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for TimelineEventContentElevenOneDocContentItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TimelineEventContentElevenOneDocContentItemZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentElevenOneDocContentItemOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentElevenOneDocContentItemType(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
