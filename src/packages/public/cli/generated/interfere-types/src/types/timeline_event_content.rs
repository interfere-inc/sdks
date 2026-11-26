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

        TimelineEventContentCorrection(TimelineEventContentCorrection),

        TimelineEventContentAction(TimelineEventContentAction),

        TimelineEventContentChart(TimelineEventContentChart),

        TimelineEventContentPriority(TimelineEventContentPriority),

        TimelineEventContentTen(TimelineEventContentTen),

        TimelineEventContentEleven(TimelineEventContentEleven),

        TimelineEventContentTwelve(TimelineEventContentTwelve),

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

    pub fn is_timeline_event_content_correction(&self) -> bool {
        matches!(self, Self::TimelineEventContentCorrection(_))
    }

    pub fn is_timeline_event_content_action(&self) -> bool {
        matches!(self, Self::TimelineEventContentAction(_))
    }

    pub fn is_timeline_event_content_chart(&self) -> bool {
        matches!(self, Self::TimelineEventContentChart(_))
    }

    pub fn is_timeline_event_content_priority(&self) -> bool {
        matches!(self, Self::TimelineEventContentPriority(_))
    }

    pub fn is_timeline_event_content_ten(&self) -> bool {
        matches!(self, Self::TimelineEventContentTen(_))
    }

    pub fn is_timeline_event_content_eleven(&self) -> bool {
        matches!(self, Self::TimelineEventContentEleven(_))
    }

    pub fn is_timeline_event_content_twelve(&self) -> bool {
        matches!(self, Self::TimelineEventContentTwelve(_))
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

    pub fn as_timeline_event_content_correction(&self) -> Option<&TimelineEventContentCorrection> {
        match self {
                    Self::TimelineEventContentCorrection(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_correction(self) -> Option<TimelineEventContentCorrection> {
        match self {
                    Self::TimelineEventContentCorrection(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_action(&self) -> Option<&TimelineEventContentAction> {
        match self {
                    Self::TimelineEventContentAction(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_action(self) -> Option<TimelineEventContentAction> {
        match self {
                    Self::TimelineEventContentAction(value) => Some(value),
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

    pub fn as_timeline_event_content_priority(&self) -> Option<&TimelineEventContentPriority> {
        match self {
                    Self::TimelineEventContentPriority(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_priority(self) -> Option<TimelineEventContentPriority> {
        match self {
                    Self::TimelineEventContentPriority(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_ten(&self) -> Option<&TimelineEventContentTen> {
        match self {
                    Self::TimelineEventContentTen(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_ten(self) -> Option<TimelineEventContentTen> {
        match self {
                    Self::TimelineEventContentTen(value) => Some(value),
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

    pub fn as_timeline_event_content_twelve(&self) -> Option<&TimelineEventContentTwelve> {
        match self {
                    Self::TimelineEventContentTwelve(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_twelve(self) -> Option<TimelineEventContentTwelve> {
        match self {
                    Self::TimelineEventContentTwelve(value) => Some(value),
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
            Self::TimelineEventContentCorrection(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentAction(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentChart(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentPriority(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentTen(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentEleven(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentTwelve(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentRelease(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentDismissal(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentTitle(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
