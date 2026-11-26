pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum TimelineEventContentTwelveCauseDocContentItemZeroContentItem {
        TimelineEventContentTwelveCauseDocContentItemZeroContentItemMarks(TimelineEventContentTwelveCauseDocContentItemZeroContentItemMarks),

        TimelineEventContentTwelveCauseDocContentItemZeroContentItemThree(TimelineEventContentTwelveCauseDocContentItemZeroContentItemThree),

        TimelineEventContentTwelveCauseDocContentItemZeroContentItemNine(TimelineEventContentTwelveCauseDocContentItemZeroContentItemNine),
}

impl TimelineEventContentTwelveCauseDocContentItemZeroContentItem {
    pub fn is_timeline_event_content_twelve_cause_doc_content_item_zero_content_item_marks(&self) -> bool {
        matches!(self, Self::TimelineEventContentTwelveCauseDocContentItemZeroContentItemMarks(_))
    }

    pub fn is_timeline_event_content_twelve_cause_doc_content_item_zero_content_item_three(&self) -> bool {
        matches!(self, Self::TimelineEventContentTwelveCauseDocContentItemZeroContentItemThree(_))
    }

    pub fn is_timeline_event_content_twelve_cause_doc_content_item_zero_content_item_nine(&self) -> bool {
        matches!(self, Self::TimelineEventContentTwelveCauseDocContentItemZeroContentItemNine(_))
    }


    pub fn as_timeline_event_content_twelve_cause_doc_content_item_zero_content_item_marks(&self) -> Option<&TimelineEventContentTwelveCauseDocContentItemZeroContentItemMarks> {
        match self {
                    Self::TimelineEventContentTwelveCauseDocContentItemZeroContentItemMarks(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_twelve_cause_doc_content_item_zero_content_item_marks(self) -> Option<TimelineEventContentTwelveCauseDocContentItemZeroContentItemMarks> {
        match self {
                    Self::TimelineEventContentTwelveCauseDocContentItemZeroContentItemMarks(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_twelve_cause_doc_content_item_zero_content_item_three(&self) -> Option<&TimelineEventContentTwelveCauseDocContentItemZeroContentItemThree> {
        match self {
                    Self::TimelineEventContentTwelveCauseDocContentItemZeroContentItemThree(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_twelve_cause_doc_content_item_zero_content_item_three(self) -> Option<TimelineEventContentTwelveCauseDocContentItemZeroContentItemThree> {
        match self {
                    Self::TimelineEventContentTwelveCauseDocContentItemZeroContentItemThree(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_twelve_cause_doc_content_item_zero_content_item_nine(&self) -> Option<&TimelineEventContentTwelveCauseDocContentItemZeroContentItemNine> {
        match self {
                    Self::TimelineEventContentTwelveCauseDocContentItemZeroContentItemNine(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_twelve_cause_doc_content_item_zero_content_item_nine(self) -> Option<TimelineEventContentTwelveCauseDocContentItemZeroContentItemNine> {
        match self {
                    Self::TimelineEventContentTwelveCauseDocContentItemZeroContentItemNine(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for TimelineEventContentTwelveCauseDocContentItemZeroContentItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TimelineEventContentTwelveCauseDocContentItemZeroContentItemMarks(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentTwelveCauseDocContentItemZeroContentItemThree(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentTwelveCauseDocContentItemZeroContentItemNine(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
