pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum TimelineEventContentTen {
        StringToValueMap(HashMap<String, serde_json::Value>),

        TimelineEventContentTenConfirmationId(TimelineEventContentTenConfirmationId),

        TimelineEventContentTenAffectedUsers(TimelineEventContentTenAffectedUsers),
}

impl TimelineEventContentTen {
    pub fn is_string_to_value_map(&self) -> bool {
        matches!(self, Self::StringToValueMap(_))
    }

    pub fn is_timeline_event_content_ten_confirmation_id(&self) -> bool {
        matches!(self, Self::TimelineEventContentTenConfirmationId(_))
    }

    pub fn is_timeline_event_content_ten_affected_users(&self) -> bool {
        matches!(self, Self::TimelineEventContentTenAffectedUsers(_))
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

    pub fn as_timeline_event_content_ten_confirmation_id(&self) -> Option<&TimelineEventContentTenConfirmationId> {
        match self {
                    Self::TimelineEventContentTenConfirmationId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_ten_confirmation_id(self) -> Option<TimelineEventContentTenConfirmationId> {
        match self {
                    Self::TimelineEventContentTenConfirmationId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_ten_affected_users(&self) -> Option<&TimelineEventContentTenAffectedUsers> {
        match self {
                    Self::TimelineEventContentTenAffectedUsers(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_ten_affected_users(self) -> Option<TimelineEventContentTenAffectedUsers> {
        match self {
                    Self::TimelineEventContentTenAffectedUsers(value) => Some(value),
                    _ => None,
                }
    }
}
