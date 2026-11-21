pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum TimelineEventContent {
        TimelineEventContentActionKind(TimelineEventContentActionKind),

        TimelineEventContentEvidenceGap(TimelineEventContentEvidenceGap),

        TimelineEventContentNewAssigneeGroupId(TimelineEventContentNewAssigneeGroupId),

        TimelineEventContentCategory(TimelineEventContentCategory),

        TimelineEventContentFour(TimelineEventContentFour),

        TimelineEventContentDescription(TimelineEventContentDescription),

        TimelineEventContentSix(TimelineEventContentSix),

        TimelineEventContentChart(TimelineEventContentChart),

        TimelineEventContentBasis(TimelineEventContentBasis),

        TimelineEventContentNine(TimelineEventContentNine),

        TimelineEventContentCosineScore(TimelineEventContentCosineScore),

        TimelineEventContentEleven(TimelineEventContentEleven),

        TimelineEventContentRelease(TimelineEventContentRelease),

        TimelineEventContentDismissal(TimelineEventContentDismissal),

        TimelineEventContentTitle(TimelineEventContentTitle),
}

impl TimelineEventContent {
    pub fn is_timeline_event_content_action_kind(&self) -> bool {
        matches!(self, Self::TimelineEventContentActionKind(_))
    }

    pub fn is_timeline_event_content_evidence_gap(&self) -> bool {
        matches!(self, Self::TimelineEventContentEvidenceGap(_))
    }

    pub fn is_timeline_event_content_new_assignee_group_id(&self) -> bool {
        matches!(self, Self::TimelineEventContentNewAssigneeGroupId(_))
    }

    pub fn is_timeline_event_content_category(&self) -> bool {
        matches!(self, Self::TimelineEventContentCategory(_))
    }

    pub fn is_timeline_event_content_four(&self) -> bool {
        matches!(self, Self::TimelineEventContentFour(_))
    }

    pub fn is_timeline_event_content_description(&self) -> bool {
        matches!(self, Self::TimelineEventContentDescription(_))
    }

    pub fn is_timeline_event_content_six(&self) -> bool {
        matches!(self, Self::TimelineEventContentSix(_))
    }

    pub fn is_timeline_event_content_chart(&self) -> bool {
        matches!(self, Self::TimelineEventContentChart(_))
    }

    pub fn is_timeline_event_content_basis(&self) -> bool {
        matches!(self, Self::TimelineEventContentBasis(_))
    }

    pub fn is_timeline_event_content_nine(&self) -> bool {
        matches!(self, Self::TimelineEventContentNine(_))
    }

    pub fn is_timeline_event_content_cosine_score(&self) -> bool {
        matches!(self, Self::TimelineEventContentCosineScore(_))
    }

    pub fn is_timeline_event_content_eleven(&self) -> bool {
        matches!(self, Self::TimelineEventContentEleven(_))
    }

    pub fn is_timeline_event_content_release(&self) -> bool {
        matches!(self, Self::TimelineEventContentRelease(_))
    }

    pub fn is_timeline_event_content_dismissal(&self) -> bool {
        matches!(self, Self::TimelineEventContentDismissal(_))
    }

    pub fn is_timeline_event_content_title(&self) -> bool {
        matches!(self, Self::TimelineEventContentTitle(_))
    }


    pub fn as_timeline_event_content_action_kind(&self) -> Option<&TimelineEventContentActionKind> {
        match self {
                    Self::TimelineEventContentActionKind(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_action_kind(self) -> Option<TimelineEventContentActionKind> {
        match self {
                    Self::TimelineEventContentActionKind(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_evidence_gap(&self) -> Option<&TimelineEventContentEvidenceGap> {
        match self {
                    Self::TimelineEventContentEvidenceGap(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_evidence_gap(self) -> Option<TimelineEventContentEvidenceGap> {
        match self {
                    Self::TimelineEventContentEvidenceGap(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_new_assignee_group_id(&self) -> Option<&TimelineEventContentNewAssigneeGroupId> {
        match self {
                    Self::TimelineEventContentNewAssigneeGroupId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_new_assignee_group_id(self) -> Option<TimelineEventContentNewAssigneeGroupId> {
        match self {
                    Self::TimelineEventContentNewAssigneeGroupId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_category(&self) -> Option<&TimelineEventContentCategory> {
        match self {
                    Self::TimelineEventContentCategory(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_category(self) -> Option<TimelineEventContentCategory> {
        match self {
                    Self::TimelineEventContentCategory(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_four(&self) -> Option<&TimelineEventContentFour> {
        match self {
                    Self::TimelineEventContentFour(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_four(self) -> Option<TimelineEventContentFour> {
        match self {
                    Self::TimelineEventContentFour(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_description(&self) -> Option<&TimelineEventContentDescription> {
        match self {
                    Self::TimelineEventContentDescription(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_description(self) -> Option<TimelineEventContentDescription> {
        match self {
                    Self::TimelineEventContentDescription(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_six(&self) -> Option<&TimelineEventContentSix> {
        match self {
                    Self::TimelineEventContentSix(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_six(self) -> Option<TimelineEventContentSix> {
        match self {
                    Self::TimelineEventContentSix(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_chart(&self) -> Option<&TimelineEventContentChart> {
        match self {
                    Self::TimelineEventContentChart(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_chart(self) -> Option<TimelineEventContentChart> {
        match self {
                    Self::TimelineEventContentChart(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_basis(&self) -> Option<&TimelineEventContentBasis> {
        match self {
                    Self::TimelineEventContentBasis(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_basis(self) -> Option<TimelineEventContentBasis> {
        match self {
                    Self::TimelineEventContentBasis(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_nine(&self) -> Option<&TimelineEventContentNine> {
        match self {
                    Self::TimelineEventContentNine(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_nine(self) -> Option<TimelineEventContentNine> {
        match self {
                    Self::TimelineEventContentNine(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_cosine_score(&self) -> Option<&TimelineEventContentCosineScore> {
        match self {
                    Self::TimelineEventContentCosineScore(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_cosine_score(self) -> Option<TimelineEventContentCosineScore> {
        match self {
                    Self::TimelineEventContentCosineScore(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_eleven(&self) -> Option<&TimelineEventContentEleven> {
        match self {
                    Self::TimelineEventContentEleven(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_eleven(self) -> Option<TimelineEventContentEleven> {
        match self {
                    Self::TimelineEventContentEleven(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_release(&self) -> Option<&TimelineEventContentRelease> {
        match self {
                    Self::TimelineEventContentRelease(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_release(self) -> Option<TimelineEventContentRelease> {
        match self {
                    Self::TimelineEventContentRelease(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_dismissal(&self) -> Option<&TimelineEventContentDismissal> {
        match self {
                    Self::TimelineEventContentDismissal(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_dismissal(self) -> Option<TimelineEventContentDismissal> {
        match self {
                    Self::TimelineEventContentDismissal(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_title(&self) -> Option<&TimelineEventContentTitle> {
        match self {
                    Self::TimelineEventContentTitle(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_title(self) -> Option<TimelineEventContentTitle> {
        match self {
                    Self::TimelineEventContentTitle(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for TimelineEventContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TimelineEventContentActionKind(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentEvidenceGap(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentNewAssigneeGroupId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentCategory(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentFour(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentDescription(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentSix(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentChart(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentBasis(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentNine(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentCosineScore(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentEleven(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentRelease(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentDismissal(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentTitle(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
