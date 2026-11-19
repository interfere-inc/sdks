pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum TimelineEventContentDismissalDismissal {
        TimelineEventContentDismissalDismissalZero(TimelineEventContentDismissalDismissalZero),

        TimelineEventContentDismissalDismissalProblemId(TimelineEventContentDismissalDismissalProblemId),
}

impl TimelineEventContentDismissalDismissal {
    pub fn is_timeline_event_content_dismissal_dismissal_zero(&self) -> bool {
        matches!(self, Self::TimelineEventContentDismissalDismissalZero(_))
    }

    pub fn is_timeline_event_content_dismissal_dismissal_problem_id(&self) -> bool {
        matches!(self, Self::TimelineEventContentDismissalDismissalProblemId(_))
    }


    pub fn as_timeline_event_content_dismissal_dismissal_zero(&self) -> Option<&TimelineEventContentDismissalDismissalZero> {
        match self {
                    Self::TimelineEventContentDismissalDismissalZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_dismissal_dismissal_zero(self) -> Option<TimelineEventContentDismissalDismissalZero> {
        match self {
                    Self::TimelineEventContentDismissalDismissalZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_dismissal_dismissal_problem_id(&self) -> Option<&TimelineEventContentDismissalDismissalProblemId> {
        match self {
                    Self::TimelineEventContentDismissalDismissalProblemId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_dismissal_dismissal_problem_id(self) -> Option<TimelineEventContentDismissalDismissalProblemId> {
        match self {
                    Self::TimelineEventContentDismissalDismissalProblemId(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for TimelineEventContentDismissalDismissal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TimelineEventContentDismissalDismissalZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentDismissalDismissalProblemId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
