pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum TimelineEventContentFourDocContentItemZeroContentItem {
        TimelineEventContentFourDocContentItemZeroContentItemMarks(TimelineEventContentFourDocContentItemZeroContentItemMarks),

        TimelineEventContentFourDocContentItemZeroContentItemOne(TimelineEventContentFourDocContentItemZeroContentItemOne),

        TimelineEventContentFourDocContentItemZeroContentItemTwo(TimelineEventContentFourDocContentItemZeroContentItemTwo),

        TimelineEventContentFourDocContentItemZeroContentItemThree(TimelineEventContentFourDocContentItemZeroContentItemThree),

        TimelineEventContentFourDocContentItemZeroContentItemFour(TimelineEventContentFourDocContentItemZeroContentItemFour),

        TimelineEventContentFourDocContentItemZeroContentItemFive(TimelineEventContentFourDocContentItemZeroContentItemFive),

        TimelineEventContentFourDocContentItemZeroContentItemSix(TimelineEventContentFourDocContentItemZeroContentItemSix),

        TimelineEventContentFourDocContentItemZeroContentItemSeven(TimelineEventContentFourDocContentItemZeroContentItemSeven),

        TimelineEventContentFourDocContentItemZeroContentItemEight(TimelineEventContentFourDocContentItemZeroContentItemEight),

        TimelineEventContentFourDocContentItemZeroContentItemNine(TimelineEventContentFourDocContentItemZeroContentItemNine),
}

impl TimelineEventContentFourDocContentItemZeroContentItem {
    pub fn is_timeline_event_content_four_doc_content_item_zero_content_item_marks(&self) -> bool {
        matches!(self, Self::TimelineEventContentFourDocContentItemZeroContentItemMarks(_))
    }

    pub fn is_timeline_event_content_four_doc_content_item_zero_content_item_one(&self) -> bool {
        matches!(self, Self::TimelineEventContentFourDocContentItemZeroContentItemOne(_))
    }

    pub fn is_timeline_event_content_four_doc_content_item_zero_content_item_two(&self) -> bool {
        matches!(self, Self::TimelineEventContentFourDocContentItemZeroContentItemTwo(_))
    }

    pub fn is_timeline_event_content_four_doc_content_item_zero_content_item_three(&self) -> bool {
        matches!(self, Self::TimelineEventContentFourDocContentItemZeroContentItemThree(_))
    }

    pub fn is_timeline_event_content_four_doc_content_item_zero_content_item_four(&self) -> bool {
        matches!(self, Self::TimelineEventContentFourDocContentItemZeroContentItemFour(_))
    }

    pub fn is_timeline_event_content_four_doc_content_item_zero_content_item_five(&self) -> bool {
        matches!(self, Self::TimelineEventContentFourDocContentItemZeroContentItemFive(_))
    }

    pub fn is_timeline_event_content_four_doc_content_item_zero_content_item_six(&self) -> bool {
        matches!(self, Self::TimelineEventContentFourDocContentItemZeroContentItemSix(_))
    }

    pub fn is_timeline_event_content_four_doc_content_item_zero_content_item_seven(&self) -> bool {
        matches!(self, Self::TimelineEventContentFourDocContentItemZeroContentItemSeven(_))
    }

    pub fn is_timeline_event_content_four_doc_content_item_zero_content_item_eight(&self) -> bool {
        matches!(self, Self::TimelineEventContentFourDocContentItemZeroContentItemEight(_))
    }

    pub fn is_timeline_event_content_four_doc_content_item_zero_content_item_nine(&self) -> bool {
        matches!(self, Self::TimelineEventContentFourDocContentItemZeroContentItemNine(_))
    }


    pub fn as_timeline_event_content_four_doc_content_item_zero_content_item_marks(&self) -> Option<&TimelineEventContentFourDocContentItemZeroContentItemMarks> {
        match self {
                    Self::TimelineEventContentFourDocContentItemZeroContentItemMarks(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_four_doc_content_item_zero_content_item_marks(self) -> Option<TimelineEventContentFourDocContentItemZeroContentItemMarks> {
        match self {
                    Self::TimelineEventContentFourDocContentItemZeroContentItemMarks(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_four_doc_content_item_zero_content_item_one(&self) -> Option<&TimelineEventContentFourDocContentItemZeroContentItemOne> {
        match self {
                    Self::TimelineEventContentFourDocContentItemZeroContentItemOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_four_doc_content_item_zero_content_item_one(self) -> Option<TimelineEventContentFourDocContentItemZeroContentItemOne> {
        match self {
                    Self::TimelineEventContentFourDocContentItemZeroContentItemOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_four_doc_content_item_zero_content_item_two(&self) -> Option<&TimelineEventContentFourDocContentItemZeroContentItemTwo> {
        match self {
                    Self::TimelineEventContentFourDocContentItemZeroContentItemTwo(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_four_doc_content_item_zero_content_item_two(self) -> Option<TimelineEventContentFourDocContentItemZeroContentItemTwo> {
        match self {
                    Self::TimelineEventContentFourDocContentItemZeroContentItemTwo(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_four_doc_content_item_zero_content_item_three(&self) -> Option<&TimelineEventContentFourDocContentItemZeroContentItemThree> {
        match self {
                    Self::TimelineEventContentFourDocContentItemZeroContentItemThree(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_four_doc_content_item_zero_content_item_three(self) -> Option<TimelineEventContentFourDocContentItemZeroContentItemThree> {
        match self {
                    Self::TimelineEventContentFourDocContentItemZeroContentItemThree(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_four_doc_content_item_zero_content_item_four(&self) -> Option<&TimelineEventContentFourDocContentItemZeroContentItemFour> {
        match self {
                    Self::TimelineEventContentFourDocContentItemZeroContentItemFour(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_four_doc_content_item_zero_content_item_four(self) -> Option<TimelineEventContentFourDocContentItemZeroContentItemFour> {
        match self {
                    Self::TimelineEventContentFourDocContentItemZeroContentItemFour(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_four_doc_content_item_zero_content_item_five(&self) -> Option<&TimelineEventContentFourDocContentItemZeroContentItemFive> {
        match self {
                    Self::TimelineEventContentFourDocContentItemZeroContentItemFive(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_four_doc_content_item_zero_content_item_five(self) -> Option<TimelineEventContentFourDocContentItemZeroContentItemFive> {
        match self {
                    Self::TimelineEventContentFourDocContentItemZeroContentItemFive(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_four_doc_content_item_zero_content_item_six(&self) -> Option<&TimelineEventContentFourDocContentItemZeroContentItemSix> {
        match self {
                    Self::TimelineEventContentFourDocContentItemZeroContentItemSix(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_four_doc_content_item_zero_content_item_six(self) -> Option<TimelineEventContentFourDocContentItemZeroContentItemSix> {
        match self {
                    Self::TimelineEventContentFourDocContentItemZeroContentItemSix(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_four_doc_content_item_zero_content_item_seven(&self) -> Option<&TimelineEventContentFourDocContentItemZeroContentItemSeven> {
        match self {
                    Self::TimelineEventContentFourDocContentItemZeroContentItemSeven(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_four_doc_content_item_zero_content_item_seven(self) -> Option<TimelineEventContentFourDocContentItemZeroContentItemSeven> {
        match self {
                    Self::TimelineEventContentFourDocContentItemZeroContentItemSeven(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_four_doc_content_item_zero_content_item_eight(&self) -> Option<&TimelineEventContentFourDocContentItemZeroContentItemEight> {
        match self {
                    Self::TimelineEventContentFourDocContentItemZeroContentItemEight(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_four_doc_content_item_zero_content_item_eight(self) -> Option<TimelineEventContentFourDocContentItemZeroContentItemEight> {
        match self {
                    Self::TimelineEventContentFourDocContentItemZeroContentItemEight(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_four_doc_content_item_zero_content_item_nine(&self) -> Option<&TimelineEventContentFourDocContentItemZeroContentItemNine> {
        match self {
                    Self::TimelineEventContentFourDocContentItemZeroContentItemNine(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_four_doc_content_item_zero_content_item_nine(self) -> Option<TimelineEventContentFourDocContentItemZeroContentItemNine> {
        match self {
                    Self::TimelineEventContentFourDocContentItemZeroContentItemNine(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for TimelineEventContentFourDocContentItemZeroContentItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TimelineEventContentFourDocContentItemZeroContentItemMarks(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentFourDocContentItemZeroContentItemOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentFourDocContentItemZeroContentItemTwo(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentFourDocContentItemZeroContentItemThree(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentFourDocContentItemZeroContentItemFour(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentFourDocContentItemZeroContentItemFive(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentFourDocContentItemZeroContentItemSix(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentFourDocContentItemZeroContentItemSeven(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentFourDocContentItemZeroContentItemEight(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentFourDocContentItemZeroContentItemNine(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
