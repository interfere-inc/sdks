pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum TimelineEventContentEvidenceGapDocContentItemZeroContentItem {
        TimelineEventContentEvidenceGapDocContentItemZeroContentItemMarks(TimelineEventContentEvidenceGapDocContentItemZeroContentItemMarks),

        TimelineEventContentEvidenceGapDocContentItemZeroContentItemOne(TimelineEventContentEvidenceGapDocContentItemZeroContentItemOne),

        TimelineEventContentEvidenceGapDocContentItemZeroContentItemTwo(TimelineEventContentEvidenceGapDocContentItemZeroContentItemTwo),

        TimelineEventContentEvidenceGapDocContentItemZeroContentItemThree(TimelineEventContentEvidenceGapDocContentItemZeroContentItemThree),

        TimelineEventContentEvidenceGapDocContentItemZeroContentItemFour(TimelineEventContentEvidenceGapDocContentItemZeroContentItemFour),

        TimelineEventContentEvidenceGapDocContentItemZeroContentItemFive(TimelineEventContentEvidenceGapDocContentItemZeroContentItemFive),

        TimelineEventContentEvidenceGapDocContentItemZeroContentItemSix(TimelineEventContentEvidenceGapDocContentItemZeroContentItemSix),

        TimelineEventContentEvidenceGapDocContentItemZeroContentItemSeven(TimelineEventContentEvidenceGapDocContentItemZeroContentItemSeven),

        TimelineEventContentEvidenceGapDocContentItemZeroContentItemEight(TimelineEventContentEvidenceGapDocContentItemZeroContentItemEight),

        TimelineEventContentEvidenceGapDocContentItemZeroContentItemNine(TimelineEventContentEvidenceGapDocContentItemZeroContentItemNine),
}

impl TimelineEventContentEvidenceGapDocContentItemZeroContentItem {
    pub fn is_timeline_event_content_evidence_gap_doc_content_item_zero_content_item_marks(&self) -> bool {
        matches!(self, Self::TimelineEventContentEvidenceGapDocContentItemZeroContentItemMarks(_))
    }

    pub fn is_timeline_event_content_evidence_gap_doc_content_item_zero_content_item_one(&self) -> bool {
        matches!(self, Self::TimelineEventContentEvidenceGapDocContentItemZeroContentItemOne(_))
    }

    pub fn is_timeline_event_content_evidence_gap_doc_content_item_zero_content_item_two(&self) -> bool {
        matches!(self, Self::TimelineEventContentEvidenceGapDocContentItemZeroContentItemTwo(_))
    }

    pub fn is_timeline_event_content_evidence_gap_doc_content_item_zero_content_item_three(&self) -> bool {
        matches!(self, Self::TimelineEventContentEvidenceGapDocContentItemZeroContentItemThree(_))
    }

    pub fn is_timeline_event_content_evidence_gap_doc_content_item_zero_content_item_four(&self) -> bool {
        matches!(self, Self::TimelineEventContentEvidenceGapDocContentItemZeroContentItemFour(_))
    }

    pub fn is_timeline_event_content_evidence_gap_doc_content_item_zero_content_item_five(&self) -> bool {
        matches!(self, Self::TimelineEventContentEvidenceGapDocContentItemZeroContentItemFive(_))
    }

    pub fn is_timeline_event_content_evidence_gap_doc_content_item_zero_content_item_six(&self) -> bool {
        matches!(self, Self::TimelineEventContentEvidenceGapDocContentItemZeroContentItemSix(_))
    }

    pub fn is_timeline_event_content_evidence_gap_doc_content_item_zero_content_item_seven(&self) -> bool {
        matches!(self, Self::TimelineEventContentEvidenceGapDocContentItemZeroContentItemSeven(_))
    }

    pub fn is_timeline_event_content_evidence_gap_doc_content_item_zero_content_item_eight(&self) -> bool {
        matches!(self, Self::TimelineEventContentEvidenceGapDocContentItemZeroContentItemEight(_))
    }

    pub fn is_timeline_event_content_evidence_gap_doc_content_item_zero_content_item_nine(&self) -> bool {
        matches!(self, Self::TimelineEventContentEvidenceGapDocContentItemZeroContentItemNine(_))
    }


    pub fn as_timeline_event_content_evidence_gap_doc_content_item_zero_content_item_marks(&self) -> Option<&TimelineEventContentEvidenceGapDocContentItemZeroContentItemMarks> {
        match self {
                    Self::TimelineEventContentEvidenceGapDocContentItemZeroContentItemMarks(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_evidence_gap_doc_content_item_zero_content_item_marks(self) -> Option<TimelineEventContentEvidenceGapDocContentItemZeroContentItemMarks> {
        match self {
                    Self::TimelineEventContentEvidenceGapDocContentItemZeroContentItemMarks(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_evidence_gap_doc_content_item_zero_content_item_one(&self) -> Option<&TimelineEventContentEvidenceGapDocContentItemZeroContentItemOne> {
        match self {
                    Self::TimelineEventContentEvidenceGapDocContentItemZeroContentItemOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_evidence_gap_doc_content_item_zero_content_item_one(self) -> Option<TimelineEventContentEvidenceGapDocContentItemZeroContentItemOne> {
        match self {
                    Self::TimelineEventContentEvidenceGapDocContentItemZeroContentItemOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_evidence_gap_doc_content_item_zero_content_item_two(&self) -> Option<&TimelineEventContentEvidenceGapDocContentItemZeroContentItemTwo> {
        match self {
                    Self::TimelineEventContentEvidenceGapDocContentItemZeroContentItemTwo(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_evidence_gap_doc_content_item_zero_content_item_two(self) -> Option<TimelineEventContentEvidenceGapDocContentItemZeroContentItemTwo> {
        match self {
                    Self::TimelineEventContentEvidenceGapDocContentItemZeroContentItemTwo(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_evidence_gap_doc_content_item_zero_content_item_three(&self) -> Option<&TimelineEventContentEvidenceGapDocContentItemZeroContentItemThree> {
        match self {
                    Self::TimelineEventContentEvidenceGapDocContentItemZeroContentItemThree(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_evidence_gap_doc_content_item_zero_content_item_three(self) -> Option<TimelineEventContentEvidenceGapDocContentItemZeroContentItemThree> {
        match self {
                    Self::TimelineEventContentEvidenceGapDocContentItemZeroContentItemThree(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_evidence_gap_doc_content_item_zero_content_item_four(&self) -> Option<&TimelineEventContentEvidenceGapDocContentItemZeroContentItemFour> {
        match self {
                    Self::TimelineEventContentEvidenceGapDocContentItemZeroContentItemFour(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_evidence_gap_doc_content_item_zero_content_item_four(self) -> Option<TimelineEventContentEvidenceGapDocContentItemZeroContentItemFour> {
        match self {
                    Self::TimelineEventContentEvidenceGapDocContentItemZeroContentItemFour(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_evidence_gap_doc_content_item_zero_content_item_five(&self) -> Option<&TimelineEventContentEvidenceGapDocContentItemZeroContentItemFive> {
        match self {
                    Self::TimelineEventContentEvidenceGapDocContentItemZeroContentItemFive(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_evidence_gap_doc_content_item_zero_content_item_five(self) -> Option<TimelineEventContentEvidenceGapDocContentItemZeroContentItemFive> {
        match self {
                    Self::TimelineEventContentEvidenceGapDocContentItemZeroContentItemFive(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_evidence_gap_doc_content_item_zero_content_item_six(&self) -> Option<&TimelineEventContentEvidenceGapDocContentItemZeroContentItemSix> {
        match self {
                    Self::TimelineEventContentEvidenceGapDocContentItemZeroContentItemSix(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_evidence_gap_doc_content_item_zero_content_item_six(self) -> Option<TimelineEventContentEvidenceGapDocContentItemZeroContentItemSix> {
        match self {
                    Self::TimelineEventContentEvidenceGapDocContentItemZeroContentItemSix(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_evidence_gap_doc_content_item_zero_content_item_seven(&self) -> Option<&TimelineEventContentEvidenceGapDocContentItemZeroContentItemSeven> {
        match self {
                    Self::TimelineEventContentEvidenceGapDocContentItemZeroContentItemSeven(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_evidence_gap_doc_content_item_zero_content_item_seven(self) -> Option<TimelineEventContentEvidenceGapDocContentItemZeroContentItemSeven> {
        match self {
                    Self::TimelineEventContentEvidenceGapDocContentItemZeroContentItemSeven(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_evidence_gap_doc_content_item_zero_content_item_eight(&self) -> Option<&TimelineEventContentEvidenceGapDocContentItemZeroContentItemEight> {
        match self {
                    Self::TimelineEventContentEvidenceGapDocContentItemZeroContentItemEight(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_evidence_gap_doc_content_item_zero_content_item_eight(self) -> Option<TimelineEventContentEvidenceGapDocContentItemZeroContentItemEight> {
        match self {
                    Self::TimelineEventContentEvidenceGapDocContentItemZeroContentItemEight(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_evidence_gap_doc_content_item_zero_content_item_nine(&self) -> Option<&TimelineEventContentEvidenceGapDocContentItemZeroContentItemNine> {
        match self {
                    Self::TimelineEventContentEvidenceGapDocContentItemZeroContentItemNine(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_evidence_gap_doc_content_item_zero_content_item_nine(self) -> Option<TimelineEventContentEvidenceGapDocContentItemZeroContentItemNine> {
        match self {
                    Self::TimelineEventContentEvidenceGapDocContentItemZeroContentItemNine(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for TimelineEventContentEvidenceGapDocContentItemZeroContentItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TimelineEventContentEvidenceGapDocContentItemZeroContentItemMarks(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentEvidenceGapDocContentItemZeroContentItemOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentEvidenceGapDocContentItemZeroContentItemTwo(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentEvidenceGapDocContentItemZeroContentItemThree(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentEvidenceGapDocContentItemZeroContentItemFour(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentEvidenceGapDocContentItemZeroContentItemFive(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentEvidenceGapDocContentItemZeroContentItemSix(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentEvidenceGapDocContentItemZeroContentItemSeven(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentEvidenceGapDocContentItemZeroContentItemEight(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentEvidenceGapDocContentItemZeroContentItemNine(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
