pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum SessionsListResponseCursorCursorSurfaceId {
        String(String),

        SessionsListResponseCursorCursorSurfaceIdOne(SessionsListResponseCursorCursorSurfaceIdOne),
}

impl SessionsListResponseCursorCursorSurfaceId {
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_sessions_list_response_cursor_cursor_surface_id_one(&self) -> bool {
        matches!(self, Self::SessionsListResponseCursorCursorSurfaceIdOne(_))
    }


    pub fn as_string(&self) -> Option<&str> {
        match self {
                    Self::String(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_string(self) -> Option<String> {
        match self {
                    Self::String(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_sessions_list_response_cursor_cursor_surface_id_one(&self) -> Option<&SessionsListResponseCursorCursorSurfaceIdOne> {
        match self {
                    Self::SessionsListResponseCursorCursorSurfaceIdOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_sessions_list_response_cursor_cursor_surface_id_one(self) -> Option<SessionsListResponseCursorCursorSurfaceIdOne> {
        match self {
                    Self::SessionsListResponseCursorCursorSurfaceIdOne(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for SessionsListResponseCursorCursorSurfaceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(value) => write!(f, "{}", value),
            Self::SessionsListResponseCursorCursorSurfaceIdOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
