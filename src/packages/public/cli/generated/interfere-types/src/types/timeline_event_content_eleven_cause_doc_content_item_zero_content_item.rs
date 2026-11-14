pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum TimelineEventContentElevenCauseDocContentItemZeroContentItem {
        TimelineEventContentElevenCauseDocContentItemZeroContentItemMarks(TimelineEventContentElevenCauseDocContentItemZeroContentItemMarks),

        TimelineEventContentElevenCauseDocContentItemZeroContentItemThree(TimelineEventContentElevenCauseDocContentItemZeroContentItemThree),

        TimelineEventContentElevenCauseDocContentItemZeroContentItemNine(TimelineEventContentElevenCauseDocContentItemZeroContentItemNine),
}

impl TimelineEventContentElevenCauseDocContentItemZeroContentItem {
    pub fn is_timeline_event_content_eleven_cause_doc_content_item_zero_content_item_marks(&self) -> bool {
        matches!(self, Self::TimelineEventContentElevenCauseDocContentItemZeroContentItemMarks(_))
    }

    pub fn is_timeline_event_content_eleven_cause_doc_content_item_zero_content_item_three(&self) -> bool {
        matches!(self, Self::TimelineEventContentElevenCauseDocContentItemZeroContentItemThree(_))
    }

    pub fn is_timeline_event_content_eleven_cause_doc_content_item_zero_content_item_nine(&self) -> bool {
        matches!(self, Self::TimelineEventContentElevenCauseDocContentItemZeroContentItemNine(_))
    }


    pub fn as_timeline_event_content_eleven_cause_doc_content_item_zero_content_item_marks(&self) -> Option<&TimelineEventContentElevenCauseDocContentItemZeroContentItemMarks> {
        match self {
                    Self::TimelineEventContentElevenCauseDocContentItemZeroContentItemMarks(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_eleven_cause_doc_content_item_zero_content_item_marks(self) -> Option<TimelineEventContentElevenCauseDocContentItemZeroContentItemMarks> {
        match self {
                    Self::TimelineEventContentElevenCauseDocContentItemZeroContentItemMarks(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_eleven_cause_doc_content_item_zero_content_item_three(&self) -> Option<&TimelineEventContentElevenCauseDocContentItemZeroContentItemThree> {
        match self {
                    Self::TimelineEventContentElevenCauseDocContentItemZeroContentItemThree(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_eleven_cause_doc_content_item_zero_content_item_three(self) -> Option<TimelineEventContentElevenCauseDocContentItemZeroContentItemThree> {
        match self {
                    Self::TimelineEventContentElevenCauseDocContentItemZeroContentItemThree(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_eleven_cause_doc_content_item_zero_content_item_nine(&self) -> Option<&TimelineEventContentElevenCauseDocContentItemZeroContentItemNine> {
        match self {
                    Self::TimelineEventContentElevenCauseDocContentItemZeroContentItemNine(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_eleven_cause_doc_content_item_zero_content_item_nine(self) -> Option<TimelineEventContentElevenCauseDocContentItemZeroContentItemNine> {
        match self {
                    Self::TimelineEventContentElevenCauseDocContentItemZeroContentItemNine(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for TimelineEventContentElevenCauseDocContentItemZeroContentItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TimelineEventContentElevenCauseDocContentItemZeroContentItemMarks(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentElevenCauseDocContentItemZeroContentItemThree(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentElevenCauseDocContentItemZeroContentItemNine(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
