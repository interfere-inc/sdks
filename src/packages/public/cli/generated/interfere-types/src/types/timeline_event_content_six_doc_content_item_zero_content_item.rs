pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum TimelineEventContentSixDocContentItemZeroContentItem {
        TimelineEventContentSixDocContentItemZeroContentItemMarks(TimelineEventContentSixDocContentItemZeroContentItemMarks),

        TimelineEventContentSixDocContentItemZeroContentItemOne(TimelineEventContentSixDocContentItemZeroContentItemOne),

        TimelineEventContentSixDocContentItemZeroContentItemTwo(TimelineEventContentSixDocContentItemZeroContentItemTwo),

        TimelineEventContentSixDocContentItemZeroContentItemThree(TimelineEventContentSixDocContentItemZeroContentItemThree),

        TimelineEventContentSixDocContentItemZeroContentItemFour(TimelineEventContentSixDocContentItemZeroContentItemFour),

        TimelineEventContentSixDocContentItemZeroContentItemFive(TimelineEventContentSixDocContentItemZeroContentItemFive),

        TimelineEventContentSixDocContentItemZeroContentItemSix(TimelineEventContentSixDocContentItemZeroContentItemSix),

        TimelineEventContentSixDocContentItemZeroContentItemSeven(TimelineEventContentSixDocContentItemZeroContentItemSeven),

        TimelineEventContentSixDocContentItemZeroContentItemEight(TimelineEventContentSixDocContentItemZeroContentItemEight),

        TimelineEventContentSixDocContentItemZeroContentItemNine(TimelineEventContentSixDocContentItemZeroContentItemNine),
}

impl TimelineEventContentSixDocContentItemZeroContentItem {
    pub fn is_timeline_event_content_six_doc_content_item_zero_content_item_marks(&self) -> bool {
        matches!(self, Self::TimelineEventContentSixDocContentItemZeroContentItemMarks(_))
    }

    pub fn is_timeline_event_content_six_doc_content_item_zero_content_item_one(&self) -> bool {
        matches!(self, Self::TimelineEventContentSixDocContentItemZeroContentItemOne(_))
    }

    pub fn is_timeline_event_content_six_doc_content_item_zero_content_item_two(&self) -> bool {
        matches!(self, Self::TimelineEventContentSixDocContentItemZeroContentItemTwo(_))
    }

    pub fn is_timeline_event_content_six_doc_content_item_zero_content_item_three(&self) -> bool {
        matches!(self, Self::TimelineEventContentSixDocContentItemZeroContentItemThree(_))
    }

    pub fn is_timeline_event_content_six_doc_content_item_zero_content_item_four(&self) -> bool {
        matches!(self, Self::TimelineEventContentSixDocContentItemZeroContentItemFour(_))
    }

    pub fn is_timeline_event_content_six_doc_content_item_zero_content_item_five(&self) -> bool {
        matches!(self, Self::TimelineEventContentSixDocContentItemZeroContentItemFive(_))
    }

    pub fn is_timeline_event_content_six_doc_content_item_zero_content_item_six(&self) -> bool {
        matches!(self, Self::TimelineEventContentSixDocContentItemZeroContentItemSix(_))
    }

    pub fn is_timeline_event_content_six_doc_content_item_zero_content_item_seven(&self) -> bool {
        matches!(self, Self::TimelineEventContentSixDocContentItemZeroContentItemSeven(_))
    }

    pub fn is_timeline_event_content_six_doc_content_item_zero_content_item_eight(&self) -> bool {
        matches!(self, Self::TimelineEventContentSixDocContentItemZeroContentItemEight(_))
    }

    pub fn is_timeline_event_content_six_doc_content_item_zero_content_item_nine(&self) -> bool {
        matches!(self, Self::TimelineEventContentSixDocContentItemZeroContentItemNine(_))
    }


    pub fn as_timeline_event_content_six_doc_content_item_zero_content_item_marks(&self) -> Option<&TimelineEventContentSixDocContentItemZeroContentItemMarks> {
        match self {
                    Self::TimelineEventContentSixDocContentItemZeroContentItemMarks(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_six_doc_content_item_zero_content_item_marks(self) -> Option<TimelineEventContentSixDocContentItemZeroContentItemMarks> {
        match self {
                    Self::TimelineEventContentSixDocContentItemZeroContentItemMarks(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_six_doc_content_item_zero_content_item_one(&self) -> Option<&TimelineEventContentSixDocContentItemZeroContentItemOne> {
        match self {
                    Self::TimelineEventContentSixDocContentItemZeroContentItemOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_six_doc_content_item_zero_content_item_one(self) -> Option<TimelineEventContentSixDocContentItemZeroContentItemOne> {
        match self {
                    Self::TimelineEventContentSixDocContentItemZeroContentItemOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_six_doc_content_item_zero_content_item_two(&self) -> Option<&TimelineEventContentSixDocContentItemZeroContentItemTwo> {
        match self {
                    Self::TimelineEventContentSixDocContentItemZeroContentItemTwo(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_six_doc_content_item_zero_content_item_two(self) -> Option<TimelineEventContentSixDocContentItemZeroContentItemTwo> {
        match self {
                    Self::TimelineEventContentSixDocContentItemZeroContentItemTwo(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_six_doc_content_item_zero_content_item_three(&self) -> Option<&TimelineEventContentSixDocContentItemZeroContentItemThree> {
        match self {
                    Self::TimelineEventContentSixDocContentItemZeroContentItemThree(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_six_doc_content_item_zero_content_item_three(self) -> Option<TimelineEventContentSixDocContentItemZeroContentItemThree> {
        match self {
                    Self::TimelineEventContentSixDocContentItemZeroContentItemThree(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_six_doc_content_item_zero_content_item_four(&self) -> Option<&TimelineEventContentSixDocContentItemZeroContentItemFour> {
        match self {
                    Self::TimelineEventContentSixDocContentItemZeroContentItemFour(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_six_doc_content_item_zero_content_item_four(self) -> Option<TimelineEventContentSixDocContentItemZeroContentItemFour> {
        match self {
                    Self::TimelineEventContentSixDocContentItemZeroContentItemFour(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_six_doc_content_item_zero_content_item_five(&self) -> Option<&TimelineEventContentSixDocContentItemZeroContentItemFive> {
        match self {
                    Self::TimelineEventContentSixDocContentItemZeroContentItemFive(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_six_doc_content_item_zero_content_item_five(self) -> Option<TimelineEventContentSixDocContentItemZeroContentItemFive> {
        match self {
                    Self::TimelineEventContentSixDocContentItemZeroContentItemFive(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_six_doc_content_item_zero_content_item_six(&self) -> Option<&TimelineEventContentSixDocContentItemZeroContentItemSix> {
        match self {
                    Self::TimelineEventContentSixDocContentItemZeroContentItemSix(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_six_doc_content_item_zero_content_item_six(self) -> Option<TimelineEventContentSixDocContentItemZeroContentItemSix> {
        match self {
                    Self::TimelineEventContentSixDocContentItemZeroContentItemSix(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_six_doc_content_item_zero_content_item_seven(&self) -> Option<&TimelineEventContentSixDocContentItemZeroContentItemSeven> {
        match self {
                    Self::TimelineEventContentSixDocContentItemZeroContentItemSeven(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_six_doc_content_item_zero_content_item_seven(self) -> Option<TimelineEventContentSixDocContentItemZeroContentItemSeven> {
        match self {
                    Self::TimelineEventContentSixDocContentItemZeroContentItemSeven(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_six_doc_content_item_zero_content_item_eight(&self) -> Option<&TimelineEventContentSixDocContentItemZeroContentItemEight> {
        match self {
                    Self::TimelineEventContentSixDocContentItemZeroContentItemEight(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_six_doc_content_item_zero_content_item_eight(self) -> Option<TimelineEventContentSixDocContentItemZeroContentItemEight> {
        match self {
                    Self::TimelineEventContentSixDocContentItemZeroContentItemEight(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_six_doc_content_item_zero_content_item_nine(&self) -> Option<&TimelineEventContentSixDocContentItemZeroContentItemNine> {
        match self {
                    Self::TimelineEventContentSixDocContentItemZeroContentItemNine(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_six_doc_content_item_zero_content_item_nine(self) -> Option<TimelineEventContentSixDocContentItemZeroContentItemNine> {
        match self {
                    Self::TimelineEventContentSixDocContentItemZeroContentItemNine(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for TimelineEventContentSixDocContentItemZeroContentItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TimelineEventContentSixDocContentItemZeroContentItemMarks(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentSixDocContentItemZeroContentItemOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentSixDocContentItemZeroContentItemTwo(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentSixDocContentItemZeroContentItemThree(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentSixDocContentItemZeroContentItemFour(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentSixDocContentItemZeroContentItemFive(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentSixDocContentItemZeroContentItemSix(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentSixDocContentItemZeroContentItemSeven(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentSixDocContentItemZeroContentItemEight(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentSixDocContentItemZeroContentItemNine(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
