pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum QueryConsumersGetByIdResponseSource {
        QueryConsumersGetByIdResponseSourceZero(QueryConsumersGetByIdResponseSourceZero),

        QueryConsumersGetByIdResponseSourceOne(QueryConsumersGetByIdResponseSourceOne),

        QueryConsumersGetByIdResponseSourceTwo(QueryConsumersGetByIdResponseSourceTwo),

        QueryConsumersGetByIdResponseSourceThree(QueryConsumersGetByIdResponseSourceThree),
}

impl QueryConsumersGetByIdResponseSource {
    pub fn is_query_consumers_get_by_id_response_source_zero(&self) -> bool {
        matches!(self, Self::QueryConsumersGetByIdResponseSourceZero(_))
    }

    pub fn is_query_consumers_get_by_id_response_source_one(&self) -> bool {
        matches!(self, Self::QueryConsumersGetByIdResponseSourceOne(_))
    }

    pub fn is_query_consumers_get_by_id_response_source_two(&self) -> bool {
        matches!(self, Self::QueryConsumersGetByIdResponseSourceTwo(_))
    }

    pub fn is_query_consumers_get_by_id_response_source_three(&self) -> bool {
        matches!(self, Self::QueryConsumersGetByIdResponseSourceThree(_))
    }


    pub fn as_query_consumers_get_by_id_response_source_zero(&self) -> Option<&QueryConsumersGetByIdResponseSourceZero> {
        match self {
                    Self::QueryConsumersGetByIdResponseSourceZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_consumers_get_by_id_response_source_zero(self) -> Option<QueryConsumersGetByIdResponseSourceZero> {
        match self {
                    Self::QueryConsumersGetByIdResponseSourceZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_consumers_get_by_id_response_source_one(&self) -> Option<&QueryConsumersGetByIdResponseSourceOne> {
        match self {
                    Self::QueryConsumersGetByIdResponseSourceOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_consumers_get_by_id_response_source_one(self) -> Option<QueryConsumersGetByIdResponseSourceOne> {
        match self {
                    Self::QueryConsumersGetByIdResponseSourceOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_consumers_get_by_id_response_source_two(&self) -> Option<&QueryConsumersGetByIdResponseSourceTwo> {
        match self {
                    Self::QueryConsumersGetByIdResponseSourceTwo(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_consumers_get_by_id_response_source_two(self) -> Option<QueryConsumersGetByIdResponseSourceTwo> {
        match self {
                    Self::QueryConsumersGetByIdResponseSourceTwo(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_consumers_get_by_id_response_source_three(&self) -> Option<&QueryConsumersGetByIdResponseSourceThree> {
        match self {
                    Self::QueryConsumersGetByIdResponseSourceThree(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_consumers_get_by_id_response_source_three(self) -> Option<QueryConsumersGetByIdResponseSourceThree> {
        match self {
                    Self::QueryConsumersGetByIdResponseSourceThree(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryConsumersGetByIdResponseSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryConsumersGetByIdResponseSourceZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryConsumersGetByIdResponseSourceOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryConsumersGetByIdResponseSourceTwo(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryConsumersGetByIdResponseSourceThree(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
