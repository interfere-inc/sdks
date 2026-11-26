pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum TimelineEventContentTwelveOneDocContentItemZeroContentItem {
        TimelineEventContentTwelveOneDocContentItemZeroContentItemMarks(TimelineEventContentTwelveOneDocContentItemZeroContentItemMarks),

        TimelineEventContentTwelveOneDocContentItemZeroContentItemThree(TimelineEventContentTwelveOneDocContentItemZeroContentItemThree),

        TimelineEventContentTwelveOneDocContentItemZeroContentItemNine(TimelineEventContentTwelveOneDocContentItemZeroContentItemNine),
}

impl TimelineEventContentTwelveOneDocContentItemZeroContentItem {
    pub fn is_timeline_event_content_twelve_one_doc_content_item_zero_content_item_marks(&self) -> bool {
        matches!(self, Self::TimelineEventContentTwelveOneDocContentItemZeroContentItemMarks(_))
    }

    pub fn is_timeline_event_content_twelve_one_doc_content_item_zero_content_item_three(&self) -> bool {
        matches!(self, Self::TimelineEventContentTwelveOneDocContentItemZeroContentItemThree(_))
    }

    pub fn is_timeline_event_content_twelve_one_doc_content_item_zero_content_item_nine(&self) -> bool {
        matches!(self, Self::TimelineEventContentTwelveOneDocContentItemZeroContentItemNine(_))
    }


    pub fn as_timeline_event_content_twelve_one_doc_content_item_zero_content_item_marks(&self) -> Option<&TimelineEventContentTwelveOneDocContentItemZeroContentItemMarks> {
        match self {
                    Self::TimelineEventContentTwelveOneDocContentItemZeroContentItemMarks(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_twelve_one_doc_content_item_zero_content_item_marks(self) -> Option<TimelineEventContentTwelveOneDocContentItemZeroContentItemMarks> {
        match self {
                    Self::TimelineEventContentTwelveOneDocContentItemZeroContentItemMarks(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_twelve_one_doc_content_item_zero_content_item_three(&self) -> Option<&TimelineEventContentTwelveOneDocContentItemZeroContentItemThree> {
        match self {
                    Self::TimelineEventContentTwelveOneDocContentItemZeroContentItemThree(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_twelve_one_doc_content_item_zero_content_item_three(self) -> Option<TimelineEventContentTwelveOneDocContentItemZeroContentItemThree> {
        match self {
                    Self::TimelineEventContentTwelveOneDocContentItemZeroContentItemThree(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_twelve_one_doc_content_item_zero_content_item_nine(&self) -> Option<&TimelineEventContentTwelveOneDocContentItemZeroContentItemNine> {
        match self {
                    Self::TimelineEventContentTwelveOneDocContentItemZeroContentItemNine(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_twelve_one_doc_content_item_zero_content_item_nine(self) -> Option<TimelineEventContentTwelveOneDocContentItemZeroContentItemNine> {
        match self {
                    Self::TimelineEventContentTwelveOneDocContentItemZeroContentItemNine(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for TimelineEventContentTwelveOneDocContentItemZeroContentItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TimelineEventContentTwelveOneDocContentItemZeroContentItemMarks(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentTwelveOneDocContentItemZeroContentItemThree(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentTwelveOneDocContentItemZeroContentItemNine(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
