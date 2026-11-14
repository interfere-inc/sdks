pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum TimelineEventContentChartDocContentItemZeroContentItem {
        TimelineEventContentChartDocContentItemZeroContentItemMarks(TimelineEventContentChartDocContentItemZeroContentItemMarks),

        TimelineEventContentChartDocContentItemZeroContentItemOne(TimelineEventContentChartDocContentItemZeroContentItemOne),

        TimelineEventContentChartDocContentItemZeroContentItemTwo(TimelineEventContentChartDocContentItemZeroContentItemTwo),

        TimelineEventContentChartDocContentItemZeroContentItemThree(TimelineEventContentChartDocContentItemZeroContentItemThree),

        TimelineEventContentChartDocContentItemZeroContentItemFour(TimelineEventContentChartDocContentItemZeroContentItemFour),

        TimelineEventContentChartDocContentItemZeroContentItemFive(TimelineEventContentChartDocContentItemZeroContentItemFive),

        TimelineEventContentChartDocContentItemZeroContentItemSix(TimelineEventContentChartDocContentItemZeroContentItemSix),

        TimelineEventContentChartDocContentItemZeroContentItemSeven(TimelineEventContentChartDocContentItemZeroContentItemSeven),

        TimelineEventContentChartDocContentItemZeroContentItemEight(TimelineEventContentChartDocContentItemZeroContentItemEight),

        TimelineEventContentChartDocContentItemZeroContentItemNine(TimelineEventContentChartDocContentItemZeroContentItemNine),
}

impl TimelineEventContentChartDocContentItemZeroContentItem {
    pub fn is_timeline_event_content_chart_doc_content_item_zero_content_item_marks(&self) -> bool {
        matches!(self, Self::TimelineEventContentChartDocContentItemZeroContentItemMarks(_))
    }

    pub fn is_timeline_event_content_chart_doc_content_item_zero_content_item_one(&self) -> bool {
        matches!(self, Self::TimelineEventContentChartDocContentItemZeroContentItemOne(_))
    }

    pub fn is_timeline_event_content_chart_doc_content_item_zero_content_item_two(&self) -> bool {
        matches!(self, Self::TimelineEventContentChartDocContentItemZeroContentItemTwo(_))
    }

    pub fn is_timeline_event_content_chart_doc_content_item_zero_content_item_three(&self) -> bool {
        matches!(self, Self::TimelineEventContentChartDocContentItemZeroContentItemThree(_))
    }

    pub fn is_timeline_event_content_chart_doc_content_item_zero_content_item_four(&self) -> bool {
        matches!(self, Self::TimelineEventContentChartDocContentItemZeroContentItemFour(_))
    }

    pub fn is_timeline_event_content_chart_doc_content_item_zero_content_item_five(&self) -> bool {
        matches!(self, Self::TimelineEventContentChartDocContentItemZeroContentItemFive(_))
    }

    pub fn is_timeline_event_content_chart_doc_content_item_zero_content_item_six(&self) -> bool {
        matches!(self, Self::TimelineEventContentChartDocContentItemZeroContentItemSix(_))
    }

    pub fn is_timeline_event_content_chart_doc_content_item_zero_content_item_seven(&self) -> bool {
        matches!(self, Self::TimelineEventContentChartDocContentItemZeroContentItemSeven(_))
    }

    pub fn is_timeline_event_content_chart_doc_content_item_zero_content_item_eight(&self) -> bool {
        matches!(self, Self::TimelineEventContentChartDocContentItemZeroContentItemEight(_))
    }

    pub fn is_timeline_event_content_chart_doc_content_item_zero_content_item_nine(&self) -> bool {
        matches!(self, Self::TimelineEventContentChartDocContentItemZeroContentItemNine(_))
    }


    pub fn as_timeline_event_content_chart_doc_content_item_zero_content_item_marks(&self) -> Option<&TimelineEventContentChartDocContentItemZeroContentItemMarks> {
        match self {
                    Self::TimelineEventContentChartDocContentItemZeroContentItemMarks(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_chart_doc_content_item_zero_content_item_marks(self) -> Option<TimelineEventContentChartDocContentItemZeroContentItemMarks> {
        match self {
                    Self::TimelineEventContentChartDocContentItemZeroContentItemMarks(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_chart_doc_content_item_zero_content_item_one(&self) -> Option<&TimelineEventContentChartDocContentItemZeroContentItemOne> {
        match self {
                    Self::TimelineEventContentChartDocContentItemZeroContentItemOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_chart_doc_content_item_zero_content_item_one(self) -> Option<TimelineEventContentChartDocContentItemZeroContentItemOne> {
        match self {
                    Self::TimelineEventContentChartDocContentItemZeroContentItemOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_chart_doc_content_item_zero_content_item_two(&self) -> Option<&TimelineEventContentChartDocContentItemZeroContentItemTwo> {
        match self {
                    Self::TimelineEventContentChartDocContentItemZeroContentItemTwo(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_chart_doc_content_item_zero_content_item_two(self) -> Option<TimelineEventContentChartDocContentItemZeroContentItemTwo> {
        match self {
                    Self::TimelineEventContentChartDocContentItemZeroContentItemTwo(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_chart_doc_content_item_zero_content_item_three(&self) -> Option<&TimelineEventContentChartDocContentItemZeroContentItemThree> {
        match self {
                    Self::TimelineEventContentChartDocContentItemZeroContentItemThree(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_chart_doc_content_item_zero_content_item_three(self) -> Option<TimelineEventContentChartDocContentItemZeroContentItemThree> {
        match self {
                    Self::TimelineEventContentChartDocContentItemZeroContentItemThree(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_chart_doc_content_item_zero_content_item_four(&self) -> Option<&TimelineEventContentChartDocContentItemZeroContentItemFour> {
        match self {
                    Self::TimelineEventContentChartDocContentItemZeroContentItemFour(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_chart_doc_content_item_zero_content_item_four(self) -> Option<TimelineEventContentChartDocContentItemZeroContentItemFour> {
        match self {
                    Self::TimelineEventContentChartDocContentItemZeroContentItemFour(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_chart_doc_content_item_zero_content_item_five(&self) -> Option<&TimelineEventContentChartDocContentItemZeroContentItemFive> {
        match self {
                    Self::TimelineEventContentChartDocContentItemZeroContentItemFive(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_chart_doc_content_item_zero_content_item_five(self) -> Option<TimelineEventContentChartDocContentItemZeroContentItemFive> {
        match self {
                    Self::TimelineEventContentChartDocContentItemZeroContentItemFive(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_chart_doc_content_item_zero_content_item_six(&self) -> Option<&TimelineEventContentChartDocContentItemZeroContentItemSix> {
        match self {
                    Self::TimelineEventContentChartDocContentItemZeroContentItemSix(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_chart_doc_content_item_zero_content_item_six(self) -> Option<TimelineEventContentChartDocContentItemZeroContentItemSix> {
        match self {
                    Self::TimelineEventContentChartDocContentItemZeroContentItemSix(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_chart_doc_content_item_zero_content_item_seven(&self) -> Option<&TimelineEventContentChartDocContentItemZeroContentItemSeven> {
        match self {
                    Self::TimelineEventContentChartDocContentItemZeroContentItemSeven(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_chart_doc_content_item_zero_content_item_seven(self) -> Option<TimelineEventContentChartDocContentItemZeroContentItemSeven> {
        match self {
                    Self::TimelineEventContentChartDocContentItemZeroContentItemSeven(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_chart_doc_content_item_zero_content_item_eight(&self) -> Option<&TimelineEventContentChartDocContentItemZeroContentItemEight> {
        match self {
                    Self::TimelineEventContentChartDocContentItemZeroContentItemEight(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_chart_doc_content_item_zero_content_item_eight(self) -> Option<TimelineEventContentChartDocContentItemZeroContentItemEight> {
        match self {
                    Self::TimelineEventContentChartDocContentItemZeroContentItemEight(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_chart_doc_content_item_zero_content_item_nine(&self) -> Option<&TimelineEventContentChartDocContentItemZeroContentItemNine> {
        match self {
                    Self::TimelineEventContentChartDocContentItemZeroContentItemNine(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_chart_doc_content_item_zero_content_item_nine(self) -> Option<TimelineEventContentChartDocContentItemZeroContentItemNine> {
        match self {
                    Self::TimelineEventContentChartDocContentItemZeroContentItemNine(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for TimelineEventContentChartDocContentItemZeroContentItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TimelineEventContentChartDocContentItemZeroContentItemMarks(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentChartDocContentItemZeroContentItemOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentChartDocContentItemZeroContentItemTwo(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentChartDocContentItemZeroContentItemThree(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentChartDocContentItemZeroContentItemFour(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentChartDocContentItemZeroContentItemFive(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentChartDocContentItemZeroContentItemSix(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentChartDocContentItemZeroContentItemSeven(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentChartDocContentItemZeroContentItemEight(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentChartDocContentItemZeroContentItemNine(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
