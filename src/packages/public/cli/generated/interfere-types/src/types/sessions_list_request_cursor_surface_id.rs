pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum SessionsListRequestCursorSurfaceId {
        String(String),

        SessionsListRequestCursorSurfaceIdOne(SessionsListRequestCursorSurfaceIdOne),
}

impl SessionsListRequestCursorSurfaceId {
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_sessions_list_request_cursor_surface_id_one(&self) -> bool {
        matches!(self, Self::SessionsListRequestCursorSurfaceIdOne(_))
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

    pub fn as_sessions_list_request_cursor_surface_id_one(&self) -> Option<&SessionsListRequestCursorSurfaceIdOne> {
        match self {
                    Self::SessionsListRequestCursorSurfaceIdOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_sessions_list_request_cursor_surface_id_one(self) -> Option<SessionsListRequestCursorSurfaceIdOne> {
        match self {
                    Self::SessionsListRequestCursorSurfaceIdOne(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for SessionsListRequestCursorSurfaceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(value) => write!(f, "{}", value),
            Self::SessionsListRequestCursorSurfaceIdOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
