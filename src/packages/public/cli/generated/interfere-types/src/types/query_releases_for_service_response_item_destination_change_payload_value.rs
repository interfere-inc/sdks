pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum QueryReleasesForServiceResponseItemDestinationChangePayloadValue {
        String(String),

        Double(f64),

        Boolean(bool),

        OptionalQueryReleasesForServiceResponseItemDestinationChangePayloadValueThreeItemList(Vec<Option<QueryReleasesForServiceResponseItemDestinationChangePayloadValueThreeItem>>),

        StringToOptionalQueryReleasesForServiceResponseItemDestinationChangePayloadValueFourValueMap(HashMap<String, Option<QueryReleasesForServiceResponseItemDestinationChangePayloadValueFourValue>>),
}

impl QueryReleasesForServiceResponseItemDestinationChangePayloadValue {
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_boolean(&self) -> bool {
        matches!(self, Self::Boolean(_))
    }

    pub fn is_optional_query_releases_for_service_response_item_destination_change_payload_value_three_item_list(&self) -> bool {
        matches!(self, Self::OptionalQueryReleasesForServiceResponseItemDestinationChangePayloadValueThreeItemList(_))
    }

    pub fn is_string_to_optional_query_releases_for_service_response_item_destination_change_payload_value_four_value_map(&self) -> bool {
        matches!(self, Self::StringToOptionalQueryReleasesForServiceResponseItemDestinationChangePayloadValueFourValueMap(_))
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

    pub fn as_optional_query_releases_for_service_response_item_destination_change_payload_value_three_item_list(&self) -> Option<&Vec<Option<QueryReleasesForServiceResponseItemDestinationChangePayloadValueThreeItem>>> {
        match self {
                    Self::OptionalQueryReleasesForServiceResponseItemDestinationChangePayloadValueThreeItemList(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_optional_query_releases_for_service_response_item_destination_change_payload_value_three_item_list(self) -> Option<Vec<Option<QueryReleasesForServiceResponseItemDestinationChangePayloadValueThreeItem>>> {
        match self {
                    Self::OptionalQueryReleasesForServiceResponseItemDestinationChangePayloadValueThreeItemList(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_string_to_optional_query_releases_for_service_response_item_destination_change_payload_value_four_value_map(&self) -> Option<&HashMap<String, Option<QueryReleasesForServiceResponseItemDestinationChangePayloadValueFourValue>>> {
        match self {
                    Self::StringToOptionalQueryReleasesForServiceResponseItemDestinationChangePayloadValueFourValueMap(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_string_to_optional_query_releases_for_service_response_item_destination_change_payload_value_four_value_map(self) -> Option<HashMap<String, Option<QueryReleasesForServiceResponseItemDestinationChangePayloadValueFourValue>>> {
        match self {
                    Self::StringToOptionalQueryReleasesForServiceResponseItemDestinationChangePayloadValueFourValueMap(value) => Some(value),
                    _ => None,
                }
    }
}
