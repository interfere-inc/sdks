pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum QueryTenantsGetPageQueryResponseItemConsumersItemSource {
        QueryTenantsGetPageQueryResponseItemConsumersItemSourceZero(QueryTenantsGetPageQueryResponseItemConsumersItemSourceZero),

        QueryTenantsGetPageQueryResponseItemConsumersItemSourceOne(QueryTenantsGetPageQueryResponseItemConsumersItemSourceOne),

        QueryTenantsGetPageQueryResponseItemConsumersItemSourceTwo(QueryTenantsGetPageQueryResponseItemConsumersItemSourceTwo),

        QueryTenantsGetPageQueryResponseItemConsumersItemSourceThree(QueryTenantsGetPageQueryResponseItemConsumersItemSourceThree),
}

impl QueryTenantsGetPageQueryResponseItemConsumersItemSource {
    pub fn is_query_tenants_get_page_query_response_item_consumers_item_source_zero(&self) -> bool {
        matches!(self, Self::QueryTenantsGetPageQueryResponseItemConsumersItemSourceZero(_))
    }

    pub fn is_query_tenants_get_page_query_response_item_consumers_item_source_one(&self) -> bool {
        matches!(self, Self::QueryTenantsGetPageQueryResponseItemConsumersItemSourceOne(_))
    }

    pub fn is_query_tenants_get_page_query_response_item_consumers_item_source_two(&self) -> bool {
        matches!(self, Self::QueryTenantsGetPageQueryResponseItemConsumersItemSourceTwo(_))
    }

    pub fn is_query_tenants_get_page_query_response_item_consumers_item_source_three(&self) -> bool {
        matches!(self, Self::QueryTenantsGetPageQueryResponseItemConsumersItemSourceThree(_))
    }


    pub fn as_query_tenants_get_page_query_response_item_consumers_item_source_zero(&self) -> Option<&QueryTenantsGetPageQueryResponseItemConsumersItemSourceZero> {
        match self {
                    Self::QueryTenantsGetPageQueryResponseItemConsumersItemSourceZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_tenants_get_page_query_response_item_consumers_item_source_zero(self) -> Option<QueryTenantsGetPageQueryResponseItemConsumersItemSourceZero> {
        match self {
                    Self::QueryTenantsGetPageQueryResponseItemConsumersItemSourceZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_tenants_get_page_query_response_item_consumers_item_source_one(&self) -> Option<&QueryTenantsGetPageQueryResponseItemConsumersItemSourceOne> {
        match self {
                    Self::QueryTenantsGetPageQueryResponseItemConsumersItemSourceOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_tenants_get_page_query_response_item_consumers_item_source_one(self) -> Option<QueryTenantsGetPageQueryResponseItemConsumersItemSourceOne> {
        match self {
                    Self::QueryTenantsGetPageQueryResponseItemConsumersItemSourceOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_tenants_get_page_query_response_item_consumers_item_source_two(&self) -> Option<&QueryTenantsGetPageQueryResponseItemConsumersItemSourceTwo> {
        match self {
                    Self::QueryTenantsGetPageQueryResponseItemConsumersItemSourceTwo(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_tenants_get_page_query_response_item_consumers_item_source_two(self) -> Option<QueryTenantsGetPageQueryResponseItemConsumersItemSourceTwo> {
        match self {
                    Self::QueryTenantsGetPageQueryResponseItemConsumersItemSourceTwo(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_tenants_get_page_query_response_item_consumers_item_source_three(&self) -> Option<&QueryTenantsGetPageQueryResponseItemConsumersItemSourceThree> {
        match self {
                    Self::QueryTenantsGetPageQueryResponseItemConsumersItemSourceThree(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_tenants_get_page_query_response_item_consumers_item_source_three(self) -> Option<QueryTenantsGetPageQueryResponseItemConsumersItemSourceThree> {
        match self {
                    Self::QueryTenantsGetPageQueryResponseItemConsumersItemSourceThree(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryTenantsGetPageQueryResponseItemConsumersItemSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryTenantsGetPageQueryResponseItemConsumersItemSourceZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryTenantsGetPageQueryResponseItemConsumersItemSourceOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryTenantsGetPageQueryResponseItemConsumersItemSourceTwo(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryTenantsGetPageQueryResponseItemConsumersItemSourceThree(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
