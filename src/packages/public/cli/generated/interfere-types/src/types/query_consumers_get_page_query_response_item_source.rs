pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum QueryConsumersGetPageQueryResponseItemSource {
        QueryConsumersGetPageQueryResponseItemSourceZero(QueryConsumersGetPageQueryResponseItemSourceZero),

        QueryConsumersGetPageQueryResponseItemSourceOne(QueryConsumersGetPageQueryResponseItemSourceOne),

        QueryConsumersGetPageQueryResponseItemSourceTwo(QueryConsumersGetPageQueryResponseItemSourceTwo),

        QueryConsumersGetPageQueryResponseItemSourceThree(QueryConsumersGetPageQueryResponseItemSourceThree),
}

impl QueryConsumersGetPageQueryResponseItemSource {
    pub fn is_query_consumers_get_page_query_response_item_source_zero(&self) -> bool {
        matches!(self, Self::QueryConsumersGetPageQueryResponseItemSourceZero(_))
    }

    pub fn is_query_consumers_get_page_query_response_item_source_one(&self) -> bool {
        matches!(self, Self::QueryConsumersGetPageQueryResponseItemSourceOne(_))
    }

    pub fn is_query_consumers_get_page_query_response_item_source_two(&self) -> bool {
        matches!(self, Self::QueryConsumersGetPageQueryResponseItemSourceTwo(_))
    }

    pub fn is_query_consumers_get_page_query_response_item_source_three(&self) -> bool {
        matches!(self, Self::QueryConsumersGetPageQueryResponseItemSourceThree(_))
    }


    pub fn as_query_consumers_get_page_query_response_item_source_zero(&self) -> Option<&QueryConsumersGetPageQueryResponseItemSourceZero> {
        match self {
                    Self::QueryConsumersGetPageQueryResponseItemSourceZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_consumers_get_page_query_response_item_source_zero(self) -> Option<QueryConsumersGetPageQueryResponseItemSourceZero> {
        match self {
                    Self::QueryConsumersGetPageQueryResponseItemSourceZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_consumers_get_page_query_response_item_source_one(&self) -> Option<&QueryConsumersGetPageQueryResponseItemSourceOne> {
        match self {
                    Self::QueryConsumersGetPageQueryResponseItemSourceOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_consumers_get_page_query_response_item_source_one(self) -> Option<QueryConsumersGetPageQueryResponseItemSourceOne> {
        match self {
                    Self::QueryConsumersGetPageQueryResponseItemSourceOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_consumers_get_page_query_response_item_source_two(&self) -> Option<&QueryConsumersGetPageQueryResponseItemSourceTwo> {
        match self {
                    Self::QueryConsumersGetPageQueryResponseItemSourceTwo(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_consumers_get_page_query_response_item_source_two(self) -> Option<QueryConsumersGetPageQueryResponseItemSourceTwo> {
        match self {
                    Self::QueryConsumersGetPageQueryResponseItemSourceTwo(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_consumers_get_page_query_response_item_source_three(&self) -> Option<&QueryConsumersGetPageQueryResponseItemSourceThree> {
        match self {
                    Self::QueryConsumersGetPageQueryResponseItemSourceThree(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_consumers_get_page_query_response_item_source_three(self) -> Option<QueryConsumersGetPageQueryResponseItemSourceThree> {
        match self {
                    Self::QueryConsumersGetPageQueryResponseItemSourceThree(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryConsumersGetPageQueryResponseItemSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryConsumersGetPageQueryResponseItemSourceZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryConsumersGetPageQueryResponseItemSourceOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryConsumersGetPageQueryResponseItemSourceTwo(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryConsumersGetPageQueryResponseItemSourceThree(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
