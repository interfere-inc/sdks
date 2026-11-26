pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum TimelineEventContentCorrectionDocContentItemZeroContentItem {
        TimelineEventContentCorrectionDocContentItemZeroContentItemMarks(TimelineEventContentCorrectionDocContentItemZeroContentItemMarks),

        TimelineEventContentCorrectionDocContentItemZeroContentItemOne(TimelineEventContentCorrectionDocContentItemZeroContentItemOne),

        TimelineEventContentCorrectionDocContentItemZeroContentItemTwo(TimelineEventContentCorrectionDocContentItemZeroContentItemTwo),

        TimelineEventContentCorrectionDocContentItemZeroContentItemThree(TimelineEventContentCorrectionDocContentItemZeroContentItemThree),

        TimelineEventContentCorrectionDocContentItemZeroContentItemFour(TimelineEventContentCorrectionDocContentItemZeroContentItemFour),

        TimelineEventContentCorrectionDocContentItemZeroContentItemFive(TimelineEventContentCorrectionDocContentItemZeroContentItemFive),

        TimelineEventContentCorrectionDocContentItemZeroContentItemSix(TimelineEventContentCorrectionDocContentItemZeroContentItemSix),

        TimelineEventContentCorrectionDocContentItemZeroContentItemSeven(TimelineEventContentCorrectionDocContentItemZeroContentItemSeven),

        TimelineEventContentCorrectionDocContentItemZeroContentItemEight(TimelineEventContentCorrectionDocContentItemZeroContentItemEight),

        TimelineEventContentCorrectionDocContentItemZeroContentItemNine(TimelineEventContentCorrectionDocContentItemZeroContentItemNine),
}

impl TimelineEventContentCorrectionDocContentItemZeroContentItem {
    pub fn is_timeline_event_content_correction_doc_content_item_zero_content_item_marks(&self) -> bool {
        matches!(self, Self::TimelineEventContentCorrectionDocContentItemZeroContentItemMarks(_))
    }

    pub fn is_timeline_event_content_correction_doc_content_item_zero_content_item_one(&self) -> bool {
        matches!(self, Self::TimelineEventContentCorrectionDocContentItemZeroContentItemOne(_))
    }

    pub fn is_timeline_event_content_correction_doc_content_item_zero_content_item_two(&self) -> bool {
        matches!(self, Self::TimelineEventContentCorrectionDocContentItemZeroContentItemTwo(_))
    }

    pub fn is_timeline_event_content_correction_doc_content_item_zero_content_item_three(&self) -> bool {
        matches!(self, Self::TimelineEventContentCorrectionDocContentItemZeroContentItemThree(_))
    }

    pub fn is_timeline_event_content_correction_doc_content_item_zero_content_item_four(&self) -> bool {
        matches!(self, Self::TimelineEventContentCorrectionDocContentItemZeroContentItemFour(_))
    }

    pub fn is_timeline_event_content_correction_doc_content_item_zero_content_item_five(&self) -> bool {
        matches!(self, Self::TimelineEventContentCorrectionDocContentItemZeroContentItemFive(_))
    }

    pub fn is_timeline_event_content_correction_doc_content_item_zero_content_item_six(&self) -> bool {
        matches!(self, Self::TimelineEventContentCorrectionDocContentItemZeroContentItemSix(_))
    }

    pub fn is_timeline_event_content_correction_doc_content_item_zero_content_item_seven(&self) -> bool {
        matches!(self, Self::TimelineEventContentCorrectionDocContentItemZeroContentItemSeven(_))
    }

    pub fn is_timeline_event_content_correction_doc_content_item_zero_content_item_eight(&self) -> bool {
        matches!(self, Self::TimelineEventContentCorrectionDocContentItemZeroContentItemEight(_))
    }

    pub fn is_timeline_event_content_correction_doc_content_item_zero_content_item_nine(&self) -> bool {
        matches!(self, Self::TimelineEventContentCorrectionDocContentItemZeroContentItemNine(_))
    }


    pub fn as_timeline_event_content_correction_doc_content_item_zero_content_item_marks(&self) -> Option<&TimelineEventContentCorrectionDocContentItemZeroContentItemMarks> {
        match self {
                    Self::TimelineEventContentCorrectionDocContentItemZeroContentItemMarks(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_correction_doc_content_item_zero_content_item_marks(self) -> Option<TimelineEventContentCorrectionDocContentItemZeroContentItemMarks> {
        match self {
                    Self::TimelineEventContentCorrectionDocContentItemZeroContentItemMarks(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_correction_doc_content_item_zero_content_item_one(&self) -> Option<&TimelineEventContentCorrectionDocContentItemZeroContentItemOne> {
        match self {
                    Self::TimelineEventContentCorrectionDocContentItemZeroContentItemOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_correction_doc_content_item_zero_content_item_one(self) -> Option<TimelineEventContentCorrectionDocContentItemZeroContentItemOne> {
        match self {
                    Self::TimelineEventContentCorrectionDocContentItemZeroContentItemOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_correction_doc_content_item_zero_content_item_two(&self) -> Option<&TimelineEventContentCorrectionDocContentItemZeroContentItemTwo> {
        match self {
                    Self::TimelineEventContentCorrectionDocContentItemZeroContentItemTwo(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_correction_doc_content_item_zero_content_item_two(self) -> Option<TimelineEventContentCorrectionDocContentItemZeroContentItemTwo> {
        match self {
                    Self::TimelineEventContentCorrectionDocContentItemZeroContentItemTwo(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_correction_doc_content_item_zero_content_item_three(&self) -> Option<&TimelineEventContentCorrectionDocContentItemZeroContentItemThree> {
        match self {
                    Self::TimelineEventContentCorrectionDocContentItemZeroContentItemThree(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_correction_doc_content_item_zero_content_item_three(self) -> Option<TimelineEventContentCorrectionDocContentItemZeroContentItemThree> {
        match self {
                    Self::TimelineEventContentCorrectionDocContentItemZeroContentItemThree(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_correction_doc_content_item_zero_content_item_four(&self) -> Option<&TimelineEventContentCorrectionDocContentItemZeroContentItemFour> {
        match self {
                    Self::TimelineEventContentCorrectionDocContentItemZeroContentItemFour(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_correction_doc_content_item_zero_content_item_four(self) -> Option<TimelineEventContentCorrectionDocContentItemZeroContentItemFour> {
        match self {
                    Self::TimelineEventContentCorrectionDocContentItemZeroContentItemFour(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_correction_doc_content_item_zero_content_item_five(&self) -> Option<&TimelineEventContentCorrectionDocContentItemZeroContentItemFive> {
        match self {
                    Self::TimelineEventContentCorrectionDocContentItemZeroContentItemFive(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_correction_doc_content_item_zero_content_item_five(self) -> Option<TimelineEventContentCorrectionDocContentItemZeroContentItemFive> {
        match self {
                    Self::TimelineEventContentCorrectionDocContentItemZeroContentItemFive(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_correction_doc_content_item_zero_content_item_six(&self) -> Option<&TimelineEventContentCorrectionDocContentItemZeroContentItemSix> {
        match self {
                    Self::TimelineEventContentCorrectionDocContentItemZeroContentItemSix(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_correction_doc_content_item_zero_content_item_six(self) -> Option<TimelineEventContentCorrectionDocContentItemZeroContentItemSix> {
        match self {
                    Self::TimelineEventContentCorrectionDocContentItemZeroContentItemSix(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_correction_doc_content_item_zero_content_item_seven(&self) -> Option<&TimelineEventContentCorrectionDocContentItemZeroContentItemSeven> {
        match self {
                    Self::TimelineEventContentCorrectionDocContentItemZeroContentItemSeven(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_correction_doc_content_item_zero_content_item_seven(self) -> Option<TimelineEventContentCorrectionDocContentItemZeroContentItemSeven> {
        match self {
                    Self::TimelineEventContentCorrectionDocContentItemZeroContentItemSeven(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_correction_doc_content_item_zero_content_item_eight(&self) -> Option<&TimelineEventContentCorrectionDocContentItemZeroContentItemEight> {
        match self {
                    Self::TimelineEventContentCorrectionDocContentItemZeroContentItemEight(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_correction_doc_content_item_zero_content_item_eight(self) -> Option<TimelineEventContentCorrectionDocContentItemZeroContentItemEight> {
        match self {
                    Self::TimelineEventContentCorrectionDocContentItemZeroContentItemEight(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_correction_doc_content_item_zero_content_item_nine(&self) -> Option<&TimelineEventContentCorrectionDocContentItemZeroContentItemNine> {
        match self {
                    Self::TimelineEventContentCorrectionDocContentItemZeroContentItemNine(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_correction_doc_content_item_zero_content_item_nine(self) -> Option<TimelineEventContentCorrectionDocContentItemZeroContentItemNine> {
        match self {
                    Self::TimelineEventContentCorrectionDocContentItemZeroContentItemNine(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for TimelineEventContentCorrectionDocContentItemZeroContentItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TimelineEventContentCorrectionDocContentItemZeroContentItemMarks(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentCorrectionDocContentItemZeroContentItemOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentCorrectionDocContentItemZeroContentItemTwo(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentCorrectionDocContentItemZeroContentItemThree(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentCorrectionDocContentItemZeroContentItemFour(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentCorrectionDocContentItemZeroContentItemFive(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentCorrectionDocContentItemZeroContentItemSix(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentCorrectionDocContentItemZeroContentItemSeven(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentCorrectionDocContentItemZeroContentItemEight(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentCorrectionDocContentItemZeroContentItemNine(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
