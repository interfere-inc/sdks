pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum QueryConsumersListResponseItemSource {
        QueryConsumersListResponseItemSourceZero(QueryConsumersListResponseItemSourceZero),

        QueryConsumersListResponseItemSourceOne(QueryConsumersListResponseItemSourceOne),

        QueryConsumersListResponseItemSourceTwo(QueryConsumersListResponseItemSourceTwo),

        QueryConsumersListResponseItemSourceThree(QueryConsumersListResponseItemSourceThree),
}

impl QueryConsumersListResponseItemSource {
    pub fn is_query_consumers_list_response_item_source_zero(&self) -> bool {
        matches!(self, Self::QueryConsumersListResponseItemSourceZero(_))
    }

    pub fn is_query_consumers_list_response_item_source_one(&self) -> bool {
        matches!(self, Self::QueryConsumersListResponseItemSourceOne(_))
    }

    pub fn is_query_consumers_list_response_item_source_two(&self) -> bool {
        matches!(self, Self::QueryConsumersListResponseItemSourceTwo(_))
    }

    pub fn is_query_consumers_list_response_item_source_three(&self) -> bool {
        matches!(self, Self::QueryConsumersListResponseItemSourceThree(_))
    }


    pub fn as_query_consumers_list_response_item_source_zero(&self) -> Option<&QueryConsumersListResponseItemSourceZero> {
        match self {
                    Self::QueryConsumersListResponseItemSourceZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_consumers_list_response_item_source_zero(self) -> Option<QueryConsumersListResponseItemSourceZero> {
        match self {
                    Self::QueryConsumersListResponseItemSourceZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_consumers_list_response_item_source_one(&self) -> Option<&QueryConsumersListResponseItemSourceOne> {
        match self {
                    Self::QueryConsumersListResponseItemSourceOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_consumers_list_response_item_source_one(self) -> Option<QueryConsumersListResponseItemSourceOne> {
        match self {
                    Self::QueryConsumersListResponseItemSourceOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_consumers_list_response_item_source_two(&self) -> Option<&QueryConsumersListResponseItemSourceTwo> {
        match self {
                    Self::QueryConsumersListResponseItemSourceTwo(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_consumers_list_response_item_source_two(self) -> Option<QueryConsumersListResponseItemSourceTwo> {
        match self {
                    Self::QueryConsumersListResponseItemSourceTwo(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_consumers_list_response_item_source_three(&self) -> Option<&QueryConsumersListResponseItemSourceThree> {
        match self {
                    Self::QueryConsumersListResponseItemSourceThree(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_consumers_list_response_item_source_three(self) -> Option<QueryConsumersListResponseItemSourceThree> {
        match self {
                    Self::QueryConsumersListResponseItemSourceThree(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryConsumersListResponseItemSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryConsumersListResponseItemSourceZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryConsumersListResponseItemSourceOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryConsumersListResponseItemSourceTwo(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryConsumersListResponseItemSourceThree(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
