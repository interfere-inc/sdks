pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum TimelineEventContentElevenOneDocContentItemZeroContentItem {
        TimelineEventContentElevenOneDocContentItemZeroContentItemMarks(TimelineEventContentElevenOneDocContentItemZeroContentItemMarks),

        TimelineEventContentElevenOneDocContentItemZeroContentItemThree(TimelineEventContentElevenOneDocContentItemZeroContentItemThree),

        TimelineEventContentElevenOneDocContentItemZeroContentItemNine(TimelineEventContentElevenOneDocContentItemZeroContentItemNine),
}

impl TimelineEventContentElevenOneDocContentItemZeroContentItem {
    pub fn is_timeline_event_content_eleven_one_doc_content_item_zero_content_item_marks(&self) -> bool {
        matches!(self, Self::TimelineEventContentElevenOneDocContentItemZeroContentItemMarks(_))
    }

    pub fn is_timeline_event_content_eleven_one_doc_content_item_zero_content_item_three(&self) -> bool {
        matches!(self, Self::TimelineEventContentElevenOneDocContentItemZeroContentItemThree(_))
    }

    pub fn is_timeline_event_content_eleven_one_doc_content_item_zero_content_item_nine(&self) -> bool {
        matches!(self, Self::TimelineEventContentElevenOneDocContentItemZeroContentItemNine(_))
    }


    pub fn as_timeline_event_content_eleven_one_doc_content_item_zero_content_item_marks(&self) -> Option<&TimelineEventContentElevenOneDocContentItemZeroContentItemMarks> {
        match self {
                    Self::TimelineEventContentElevenOneDocContentItemZeroContentItemMarks(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_eleven_one_doc_content_item_zero_content_item_marks(self) -> Option<TimelineEventContentElevenOneDocContentItemZeroContentItemMarks> {
        match self {
                    Self::TimelineEventContentElevenOneDocContentItemZeroContentItemMarks(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_eleven_one_doc_content_item_zero_content_item_three(&self) -> Option<&TimelineEventContentElevenOneDocContentItemZeroContentItemThree> {
        match self {
                    Self::TimelineEventContentElevenOneDocContentItemZeroContentItemThree(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_eleven_one_doc_content_item_zero_content_item_three(self) -> Option<TimelineEventContentElevenOneDocContentItemZeroContentItemThree> {
        match self {
                    Self::TimelineEventContentElevenOneDocContentItemZeroContentItemThree(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_eleven_one_doc_content_item_zero_content_item_nine(&self) -> Option<&TimelineEventContentElevenOneDocContentItemZeroContentItemNine> {
        match self {
                    Self::TimelineEventContentElevenOneDocContentItemZeroContentItemNine(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_eleven_one_doc_content_item_zero_content_item_nine(self) -> Option<TimelineEventContentElevenOneDocContentItemZeroContentItemNine> {
        match self {
                    Self::TimelineEventContentElevenOneDocContentItemZeroContentItemNine(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for TimelineEventContentElevenOneDocContentItemZeroContentItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TimelineEventContentElevenOneDocContentItemZeroContentItemMarks(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentElevenOneDocContentItemZeroContentItemThree(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentElevenOneDocContentItemZeroContentItemNine(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
