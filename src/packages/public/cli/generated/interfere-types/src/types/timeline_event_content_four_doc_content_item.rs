pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum TimelineEventContentFourDocContentItem {
        TimelineEventContentFourDocContentItemZero(TimelineEventContentFourDocContentItemZero),

        TimelineEventContentFourDocContentItemOne(TimelineEventContentFourDocContentItemOne),

        TimelineEventContentFourDocContentItemType(TimelineEventContentFourDocContentItemType),
}

impl TimelineEventContentFourDocContentItem {
    pub fn is_timeline_event_content_four_doc_content_item_zero(&self) -> bool {
        matches!(self, Self::TimelineEventContentFourDocContentItemZero(_))
    }

    pub fn is_timeline_event_content_four_doc_content_item_one(&self) -> bool {
        matches!(self, Self::TimelineEventContentFourDocContentItemOne(_))
    }

    pub fn is_timeline_event_content_four_doc_content_item_type(&self) -> bool {
        matches!(self, Self::TimelineEventContentFourDocContentItemType(_))
    }


    pub fn as_timeline_event_content_four_doc_content_item_zero(&self) -> Option<&TimelineEventContentFourDocContentItemZero> {
        match self {
                    Self::TimelineEventContentFourDocContentItemZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_four_doc_content_item_zero(self) -> Option<TimelineEventContentFourDocContentItemZero> {
        match self {
                    Self::TimelineEventContentFourDocContentItemZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_four_doc_content_item_one(&self) -> Option<&TimelineEventContentFourDocContentItemOne> {
        match self {
                    Self::TimelineEventContentFourDocContentItemOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_four_doc_content_item_one(self) -> Option<TimelineEventContentFourDocContentItemOne> {
        match self {
                    Self::TimelineEventContentFourDocContentItemOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_four_doc_content_item_type(&self) -> Option<&TimelineEventContentFourDocContentItemType> {
        match self {
                    Self::TimelineEventContentFourDocContentItemType(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_four_doc_content_item_type(self) -> Option<TimelineEventContentFourDocContentItemType> {
        match self {
                    Self::TimelineEventContentFourDocContentItemType(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for TimelineEventContentFourDocContentItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TimelineEventContentFourDocContentItemZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentFourDocContentItemOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentFourDocContentItemType(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
