pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum TimelineEventContentEleven {
        TimelineEventContentElevenCause(TimelineEventContentElevenCause),

        TimelineEventContentElevenOne(TimelineEventContentElevenOne),
}

impl TimelineEventContentEleven {
    pub fn is_timeline_event_content_eleven_cause(&self) -> bool {
        matches!(self, Self::TimelineEventContentElevenCause(_))
    }

    pub fn is_timeline_event_content_eleven_one(&self) -> bool {
        matches!(self, Self::TimelineEventContentElevenOne(_))
    }


    pub fn as_timeline_event_content_eleven_cause(&self) -> Option<&TimelineEventContentElevenCause> {
        match self {
                    Self::TimelineEventContentElevenCause(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_eleven_cause(self) -> Option<TimelineEventContentElevenCause> {
        match self {
                    Self::TimelineEventContentElevenCause(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_eleven_one(&self) -> Option<&TimelineEventContentElevenOne> {
        match self {
                    Self::TimelineEventContentElevenOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_eleven_one(self) -> Option<TimelineEventContentElevenOne> {
        match self {
                    Self::TimelineEventContentElevenOne(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for TimelineEventContentEleven {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TimelineEventContentElevenCause(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentElevenOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
