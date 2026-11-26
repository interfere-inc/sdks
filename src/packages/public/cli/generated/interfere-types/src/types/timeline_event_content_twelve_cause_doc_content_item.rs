pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum TimelineEventContentTwelveCauseDocContentItem {
        TimelineEventContentTwelveCauseDocContentItemZero(TimelineEventContentTwelveCauseDocContentItemZero),

        TimelineEventContentTwelveCauseDocContentItemOne(TimelineEventContentTwelveCauseDocContentItemOne),

        TimelineEventContentTwelveCauseDocContentItemType(TimelineEventContentTwelveCauseDocContentItemType),
}

impl TimelineEventContentTwelveCauseDocContentItem {
    pub fn is_timeline_event_content_twelve_cause_doc_content_item_zero(&self) -> bool {
        matches!(self, Self::TimelineEventContentTwelveCauseDocContentItemZero(_))
    }

    pub fn is_timeline_event_content_twelve_cause_doc_content_item_one(&self) -> bool {
        matches!(self, Self::TimelineEventContentTwelveCauseDocContentItemOne(_))
    }

    pub fn is_timeline_event_content_twelve_cause_doc_content_item_type(&self) -> bool {
        matches!(self, Self::TimelineEventContentTwelveCauseDocContentItemType(_))
    }


    pub fn as_timeline_event_content_twelve_cause_doc_content_item_zero(&self) -> Option<&TimelineEventContentTwelveCauseDocContentItemZero> {
        match self {
                    Self::TimelineEventContentTwelveCauseDocContentItemZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_twelve_cause_doc_content_item_zero(self) -> Option<TimelineEventContentTwelveCauseDocContentItemZero> {
        match self {
                    Self::TimelineEventContentTwelveCauseDocContentItemZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_twelve_cause_doc_content_item_one(&self) -> Option<&TimelineEventContentTwelveCauseDocContentItemOne> {
        match self {
                    Self::TimelineEventContentTwelveCauseDocContentItemOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_twelve_cause_doc_content_item_one(self) -> Option<TimelineEventContentTwelveCauseDocContentItemOne> {
        match self {
                    Self::TimelineEventContentTwelveCauseDocContentItemOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_twelve_cause_doc_content_item_type(&self) -> Option<&TimelineEventContentTwelveCauseDocContentItemType> {
        match self {
                    Self::TimelineEventContentTwelveCauseDocContentItemType(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_twelve_cause_doc_content_item_type(self) -> Option<TimelineEventContentTwelveCauseDocContentItemType> {
        match self {
                    Self::TimelineEventContentTwelveCauseDocContentItemType(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for TimelineEventContentTwelveCauseDocContentItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TimelineEventContentTwelveCauseDocContentItemZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentTwelveCauseDocContentItemOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentTwelveCauseDocContentItemType(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
