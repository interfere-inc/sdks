pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum QueryTenantsSeenUserResponseSource {
        QueryTenantsSeenUserResponseSourceZero(QueryTenantsSeenUserResponseSourceZero),

        QueryTenantsSeenUserResponseSourceOne(QueryTenantsSeenUserResponseSourceOne),

        QueryTenantsSeenUserResponseSourceTwo(QueryTenantsSeenUserResponseSourceTwo),

        QueryTenantsSeenUserResponseSourceThree(QueryTenantsSeenUserResponseSourceThree),
}

impl QueryTenantsSeenUserResponseSource {
    pub fn is_query_tenants_seen_user_response_source_zero(&self) -> bool {
        matches!(self, Self::QueryTenantsSeenUserResponseSourceZero(_))
    }

    pub fn is_query_tenants_seen_user_response_source_one(&self) -> bool {
        matches!(self, Self::QueryTenantsSeenUserResponseSourceOne(_))
    }

    pub fn is_query_tenants_seen_user_response_source_two(&self) -> bool {
        matches!(self, Self::QueryTenantsSeenUserResponseSourceTwo(_))
    }

    pub fn is_query_tenants_seen_user_response_source_three(&self) -> bool {
        matches!(self, Self::QueryTenantsSeenUserResponseSourceThree(_))
    }


    pub fn as_query_tenants_seen_user_response_source_zero(&self) -> Option<&QueryTenantsSeenUserResponseSourceZero> {
        match self {
                    Self::QueryTenantsSeenUserResponseSourceZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_tenants_seen_user_response_source_zero(self) -> Option<QueryTenantsSeenUserResponseSourceZero> {
        match self {
                    Self::QueryTenantsSeenUserResponseSourceZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_tenants_seen_user_response_source_one(&self) -> Option<&QueryTenantsSeenUserResponseSourceOne> {
        match self {
                    Self::QueryTenantsSeenUserResponseSourceOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_tenants_seen_user_response_source_one(self) -> Option<QueryTenantsSeenUserResponseSourceOne> {
        match self {
                    Self::QueryTenantsSeenUserResponseSourceOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_tenants_seen_user_response_source_two(&self) -> Option<&QueryTenantsSeenUserResponseSourceTwo> {
        match self {
                    Self::QueryTenantsSeenUserResponseSourceTwo(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_tenants_seen_user_response_source_two(self) -> Option<QueryTenantsSeenUserResponseSourceTwo> {
        match self {
                    Self::QueryTenantsSeenUserResponseSourceTwo(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_tenants_seen_user_response_source_three(&self) -> Option<&QueryTenantsSeenUserResponseSourceThree> {
        match self {
                    Self::QueryTenantsSeenUserResponseSourceThree(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_tenants_seen_user_response_source_three(self) -> Option<QueryTenantsSeenUserResponseSourceThree> {
        match self {
                    Self::QueryTenantsSeenUserResponseSourceThree(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryTenantsSeenUserResponseSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryTenantsSeenUserResponseSourceZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryTenantsSeenUserResponseSourceOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryTenantsSeenUserResponseSourceTwo(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryTenantsSeenUserResponseSourceThree(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
