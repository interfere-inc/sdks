pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum QueryTenantsSeenUserResponseTraitsThreeItem {
        String(String),

        Double(f64),

        Boolean(bool),

        OptionalValueList(Vec<Option<serde_json::Value>>),

        StringToValueMap(HashMap<String, serde_json::Value>),
}

impl QueryTenantsSeenUserResponseTraitsThreeItem {
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_boolean(&self) -> bool {
        matches!(self, Self::Boolean(_))
    }

    pub fn is_optional_value_list(&self) -> bool {
        matches!(self, Self::OptionalValueList(_))
    }

    pub fn is_string_to_value_map(&self) -> bool {
        matches!(self, Self::StringToValueMap(_))
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

    pub fn as_optional_value_list(&self) -> Option<&Vec<Option<serde_json::Value>>> {
        match self {
                    Self::OptionalValueList(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_optional_value_list(self) -> Option<Vec<Option<serde_json::Value>>> {
        match self {
                    Self::OptionalValueList(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_string_to_value_map(&self) -> Option<&HashMap<String, serde_json::Value>> {
        match self {
                    Self::StringToValueMap(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_string_to_value_map(self) -> Option<HashMap<String, serde_json::Value>> {
        match self {
                    Self::StringToValueMap(value) => Some(value),
                    _ => None,
                }
    }
}
