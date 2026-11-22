pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum SessionSummarySurfaceId {
        String(String),

        SessionSummarySurfaceIdOne(SessionSummarySurfaceIdOne),
}

impl SessionSummarySurfaceId {
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_session_summary_surface_id_one(&self) -> bool {
        matches!(self, Self::SessionSummarySurfaceIdOne(_))
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

    pub fn as_session_summary_surface_id_one(&self) -> Option<&SessionSummarySurfaceIdOne> {
        match self {
                    Self::SessionSummarySurfaceIdOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_session_summary_surface_id_one(self) -> Option<SessionSummarySurfaceIdOne> {
        match self {
                    Self::SessionSummarySurfaceIdOne(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for SessionSummarySurfaceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(value) => write!(f, "{}", value),
            Self::SessionSummarySurfaceIdOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
