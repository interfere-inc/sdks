pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum SessionDetailResponseSurfaceId {
        String(String),

        SessionDetailResponseSurfaceIdOne(SessionDetailResponseSurfaceIdOne),
}

impl SessionDetailResponseSurfaceId {
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_session_detail_response_surface_id_one(&self) -> bool {
        matches!(self, Self::SessionDetailResponseSurfaceIdOne(_))
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

    pub fn as_session_detail_response_surface_id_one(&self) -> Option<&SessionDetailResponseSurfaceIdOne> {
        match self {
                    Self::SessionDetailResponseSurfaceIdOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_session_detail_response_surface_id_one(self) -> Option<SessionDetailResponseSurfaceIdOne> {
        match self {
                    Self::SessionDetailResponseSurfaceIdOne(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for SessionDetailResponseSurfaceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(value) => write!(f, "{}", value),
            Self::SessionDetailResponseSurfaceIdOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
