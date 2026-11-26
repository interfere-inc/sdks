pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum TimelineEventContentTwelveOneDocContentItem {
        TimelineEventContentTwelveOneDocContentItemZero(TimelineEventContentTwelveOneDocContentItemZero),

        TimelineEventContentTwelveOneDocContentItemOne(TimelineEventContentTwelveOneDocContentItemOne),

        TimelineEventContentTwelveOneDocContentItemType(TimelineEventContentTwelveOneDocContentItemType),
}

impl TimelineEventContentTwelveOneDocContentItem {
    pub fn is_timeline_event_content_twelve_one_doc_content_item_zero(&self) -> bool {
        matches!(self, Self::TimelineEventContentTwelveOneDocContentItemZero(_))
    }

    pub fn is_timeline_event_content_twelve_one_doc_content_item_one(&self) -> bool {
        matches!(self, Self::TimelineEventContentTwelveOneDocContentItemOne(_))
    }

    pub fn is_timeline_event_content_twelve_one_doc_content_item_type(&self) -> bool {
        matches!(self, Self::TimelineEventContentTwelveOneDocContentItemType(_))
    }


    pub fn as_timeline_event_content_twelve_one_doc_content_item_zero(&self) -> Option<&TimelineEventContentTwelveOneDocContentItemZero> {
        match self {
                    Self::TimelineEventContentTwelveOneDocContentItemZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_twelve_one_doc_content_item_zero(self) -> Option<TimelineEventContentTwelveOneDocContentItemZero> {
        match self {
                    Self::TimelineEventContentTwelveOneDocContentItemZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_twelve_one_doc_content_item_one(&self) -> Option<&TimelineEventContentTwelveOneDocContentItemOne> {
        match self {
                    Self::TimelineEventContentTwelveOneDocContentItemOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_twelve_one_doc_content_item_one(self) -> Option<TimelineEventContentTwelveOneDocContentItemOne> {
        match self {
                    Self::TimelineEventContentTwelveOneDocContentItemOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_twelve_one_doc_content_item_type(&self) -> Option<&TimelineEventContentTwelveOneDocContentItemType> {
        match self {
                    Self::TimelineEventContentTwelveOneDocContentItemType(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_twelve_one_doc_content_item_type(self) -> Option<TimelineEventContentTwelveOneDocContentItemType> {
        match self {
                    Self::TimelineEventContentTwelveOneDocContentItemType(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for TimelineEventContentTwelveOneDocContentItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TimelineEventContentTwelveOneDocContentItemZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentTwelveOneDocContentItemOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentTwelveOneDocContentItemType(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
