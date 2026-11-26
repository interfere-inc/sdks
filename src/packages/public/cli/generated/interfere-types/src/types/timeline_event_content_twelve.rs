pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum TimelineEventContentTwelve {
        TimelineEventContentTwelveCause(TimelineEventContentTwelveCause),

        TimelineEventContentTwelveOne(TimelineEventContentTwelveOne),
}

impl TimelineEventContentTwelve {
    pub fn is_timeline_event_content_twelve_cause(&self) -> bool {
        matches!(self, Self::TimelineEventContentTwelveCause(_))
    }

    pub fn is_timeline_event_content_twelve_one(&self) -> bool {
        matches!(self, Self::TimelineEventContentTwelveOne(_))
    }


    pub fn as_timeline_event_content_twelve_cause(&self) -> Option<&TimelineEventContentTwelveCause> {
        match self {
                    Self::TimelineEventContentTwelveCause(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_twelve_cause(self) -> Option<TimelineEventContentTwelveCause> {
        match self {
                    Self::TimelineEventContentTwelveCause(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_twelve_one(&self) -> Option<&TimelineEventContentTwelveOne> {
        match self {
                    Self::TimelineEventContentTwelveOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_twelve_one(self) -> Option<TimelineEventContentTwelveOne> {
        match self {
                    Self::TimelineEventContentTwelveOne(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for TimelineEventContentTwelve {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TimelineEventContentTwelveCause(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentTwelveOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
