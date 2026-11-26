pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum TimelineEventContentEleven {
        TimelineEventContentElevenFixReleaseId(TimelineEventContentElevenFixReleaseId),

        TimelineEventContentElevenCosineScore(TimelineEventContentElevenCosineScore),
}

impl TimelineEventContentEleven {
    pub fn is_timeline_event_content_eleven_fix_release_id(&self) -> bool {
        matches!(self, Self::TimelineEventContentElevenFixReleaseId(_))
    }

    pub fn is_timeline_event_content_eleven_cosine_score(&self) -> bool {
        matches!(self, Self::TimelineEventContentElevenCosineScore(_))
    }


    pub fn as_timeline_event_content_eleven_fix_release_id(&self) -> Option<&TimelineEventContentElevenFixReleaseId> {
        match self {
                    Self::TimelineEventContentElevenFixReleaseId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_eleven_fix_release_id(self) -> Option<TimelineEventContentElevenFixReleaseId> {
        match self {
                    Self::TimelineEventContentElevenFixReleaseId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_eleven_cosine_score(&self) -> Option<&TimelineEventContentElevenCosineScore> {
        match self {
                    Self::TimelineEventContentElevenCosineScore(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_eleven_cosine_score(self) -> Option<TimelineEventContentElevenCosineScore> {
        match self {
                    Self::TimelineEventContentElevenCosineScore(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for TimelineEventContentEleven {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TimelineEventContentElevenFixReleaseId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentElevenCosineScore(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
