pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum TimelineEventContentNine {
        StringToValueMap(HashMap<String, serde_json::Value>),

        TimelineEventContentNineConfirmationId(TimelineEventContentNineConfirmationId),

        TimelineEventContentNineAffectedUsers(TimelineEventContentNineAffectedUsers),
}

impl TimelineEventContentNine {
    pub fn is_string_to_value_map(&self) -> bool {
        matches!(self, Self::StringToValueMap(_))
    }

    pub fn is_timeline_event_content_nine_confirmation_id(&self) -> bool {
        matches!(self, Self::TimelineEventContentNineConfirmationId(_))
    }

    pub fn is_timeline_event_content_nine_affected_users(&self) -> bool {
        matches!(self, Self::TimelineEventContentNineAffectedUsers(_))
    }


    pub fn as_string_to_value_map(&self) -> Option<&HashMap<String, serde_json::Value>> {
        match self {
                    Self::StringToValueMap(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_string_to_value_map(self) -> Option<HashMap<String, serde_json::Value>> {
        match self {
                    Self::StringToValueMap(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_nine_confirmation_id(&self) -> Option<&TimelineEventContentNineConfirmationId> {
        match self {
                    Self::TimelineEventContentNineConfirmationId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_nine_confirmation_id(self) -> Option<TimelineEventContentNineConfirmationId> {
        match self {
                    Self::TimelineEventContentNineConfirmationId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_nine_affected_users(&self) -> Option<&TimelineEventContentNineAffectedUsers> {
        match self {
                    Self::TimelineEventContentNineAffectedUsers(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_nine_affected_users(self) -> Option<TimelineEventContentNineAffectedUsers> {
        match self {
                    Self::TimelineEventContentNineAffectedUsers(value) => Some(value),
                    _ => None,
                }
    }
}
