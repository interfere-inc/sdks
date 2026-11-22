pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum ProblemSessionsResponseSessionsItemSurfaceId {
        String(String),

        ProblemSessionsResponseSessionsItemSurfaceIdOne(ProblemSessionsResponseSessionsItemSurfaceIdOne),
}

impl ProblemSessionsResponseSessionsItemSurfaceId {
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_problem_sessions_response_sessions_item_surface_id_one(&self) -> bool {
        matches!(self, Self::ProblemSessionsResponseSessionsItemSurfaceIdOne(_))
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

    pub fn as_problem_sessions_response_sessions_item_surface_id_one(&self) -> Option<&ProblemSessionsResponseSessionsItemSurfaceIdOne> {
        match self {
                    Self::ProblemSessionsResponseSessionsItemSurfaceIdOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_problem_sessions_response_sessions_item_surface_id_one(self) -> Option<ProblemSessionsResponseSessionsItemSurfaceIdOne> {
        match self {
                    Self::ProblemSessionsResponseSessionsItemSurfaceIdOne(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for ProblemSessionsResponseSessionsItemSurfaceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(value) => write!(f, "{}", value),
            Self::ProblemSessionsResponseSessionsItemSurfaceIdOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
