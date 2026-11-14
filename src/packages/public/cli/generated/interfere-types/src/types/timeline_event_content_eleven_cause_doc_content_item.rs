pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum TimelineEventContentElevenCauseDocContentItem {
        TimelineEventContentElevenCauseDocContentItemZero(TimelineEventContentElevenCauseDocContentItemZero),

        TimelineEventContentElevenCauseDocContentItemOne(TimelineEventContentElevenCauseDocContentItemOne),

        TimelineEventContentElevenCauseDocContentItemType(TimelineEventContentElevenCauseDocContentItemType),
}

impl TimelineEventContentElevenCauseDocContentItem {
    pub fn is_timeline_event_content_eleven_cause_doc_content_item_zero(&self) -> bool {
        matches!(self, Self::TimelineEventContentElevenCauseDocContentItemZero(_))
    }

    pub fn is_timeline_event_content_eleven_cause_doc_content_item_one(&self) -> bool {
        matches!(self, Self::TimelineEventContentElevenCauseDocContentItemOne(_))
    }

    pub fn is_timeline_event_content_eleven_cause_doc_content_item_type(&self) -> bool {
        matches!(self, Self::TimelineEventContentElevenCauseDocContentItemType(_))
    }


    pub fn as_timeline_event_content_eleven_cause_doc_content_item_zero(&self) -> Option<&TimelineEventContentElevenCauseDocContentItemZero> {
        match self {
                    Self::TimelineEventContentElevenCauseDocContentItemZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_eleven_cause_doc_content_item_zero(self) -> Option<TimelineEventContentElevenCauseDocContentItemZero> {
        match self {
                    Self::TimelineEventContentElevenCauseDocContentItemZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_eleven_cause_doc_content_item_one(&self) -> Option<&TimelineEventContentElevenCauseDocContentItemOne> {
        match self {
                    Self::TimelineEventContentElevenCauseDocContentItemOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_eleven_cause_doc_content_item_one(self) -> Option<TimelineEventContentElevenCauseDocContentItemOne> {
        match self {
                    Self::TimelineEventContentElevenCauseDocContentItemOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_eleven_cause_doc_content_item_type(&self) -> Option<&TimelineEventContentElevenCauseDocContentItemType> {
        match self {
                    Self::TimelineEventContentElevenCauseDocContentItemType(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_eleven_cause_doc_content_item_type(self) -> Option<TimelineEventContentElevenCauseDocContentItemType> {
        match self {
                    Self::TimelineEventContentElevenCauseDocContentItemType(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for TimelineEventContentElevenCauseDocContentItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TimelineEventContentElevenCauseDocContentItemZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentElevenCauseDocContentItemOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentElevenCauseDocContentItemType(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
