pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum QueryConsumersGetByIdResponseTraits {
        String(String),

        Double(f64),

        Boolean(bool),

        OptionalQueryConsumersGetByIdResponseTraitsThreeItemList(Vec<Option<QueryConsumersGetByIdResponseTraitsThreeItem>>),

        StringToOptionalQueryConsumersGetByIdResponseTraitsFourValueMap(HashMap<String, Option<QueryConsumersGetByIdResponseTraitsFourValue>>),
}

impl QueryConsumersGetByIdResponseTraits {
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_boolean(&self) -> bool {
        matches!(self, Self::Boolean(_))
    }

    pub fn is_optional_query_consumers_get_by_id_response_traits_three_item_list(&self) -> bool {
        matches!(self, Self::OptionalQueryConsumersGetByIdResponseTraitsThreeItemList(_))
    }

    pub fn is_string_to_optional_query_consumers_get_by_id_response_traits_four_value_map(&self) -> bool {
        matches!(self, Self::StringToOptionalQueryConsumersGetByIdResponseTraitsFourValueMap(_))
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

    pub fn as_double(&self) -> Option<&f64> {
        match self {
                    Self::Double(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_double(self) -> Option<f64> {
        match self {
                    Self::Double(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_boolean(&self) -> Option<&bool> {
        match self {
                    Self::Boolean(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_boolean(self) -> Option<bool> {
        match self {
                    Self::Boolean(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_optional_query_consumers_get_by_id_response_traits_three_item_list(&self) -> Option<&Vec<Option<QueryConsumersGetByIdResponseTraitsThreeItem>>> {
        match self {
                    Self::OptionalQueryConsumersGetByIdResponseTraitsThreeItemList(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_optional_query_consumers_get_by_id_response_traits_three_item_list(self) -> Option<Vec<Option<QueryConsumersGetByIdResponseTraitsThreeItem>>> {
        match self {
                    Self::OptionalQueryConsumersGetByIdResponseTraitsThreeItemList(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_string_to_optional_query_consumers_get_by_id_response_traits_four_value_map(&self) -> Option<&HashMap<String, Option<QueryConsumersGetByIdResponseTraitsFourValue>>> {
        match self {
                    Self::StringToOptionalQueryConsumersGetByIdResponseTraitsFourValueMap(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_string_to_optional_query_consumers_get_by_id_response_traits_four_value_map(self) -> Option<HashMap<String, Option<QueryConsumersGetByIdResponseTraitsFourValue>>> {
        match self {
                    Self::StringToOptionalQueryConsumersGetByIdResponseTraitsFourValueMap(value) => Some(value),
                    _ => None,
                }
    }
}
