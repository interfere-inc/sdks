pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum QueryReleasesForSurfaceResponseItemDestinationChangePayloadValue {
        String(String),

        Double(f64),

        Boolean(bool),

        OptionalQueryReleasesForSurfaceResponseItemDestinationChangePayloadValueThreeItemList(Vec<Option<QueryReleasesForSurfaceResponseItemDestinationChangePayloadValueThreeItem>>),

        StringToOptionalQueryReleasesForSurfaceResponseItemDestinationChangePayloadValueFourValueMap(HashMap<String, Option<QueryReleasesForSurfaceResponseItemDestinationChangePayloadValueFourValue>>),
}

impl QueryReleasesForSurfaceResponseItemDestinationChangePayloadValue {
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_boolean(&self) -> bool {
        matches!(self, Self::Boolean(_))
    }

    pub fn is_optional_query_releases_for_surface_response_item_destination_change_payload_value_three_item_list(&self) -> bool {
        matches!(self, Self::OptionalQueryReleasesForSurfaceResponseItemDestinationChangePayloadValueThreeItemList(_))
    }

    pub fn is_string_to_optional_query_releases_for_surface_response_item_destination_change_payload_value_four_value_map(&self) -> bool {
        matches!(self, Self::StringToOptionalQueryReleasesForSurfaceResponseItemDestinationChangePayloadValueFourValueMap(_))
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

    pub fn as_optional_query_releases_for_surface_response_item_destination_change_payload_value_three_item_list(&self) -> Option<&Vec<Option<QueryReleasesForSurfaceResponseItemDestinationChangePayloadValueThreeItem>>> {
        match self {
                    Self::OptionalQueryReleasesForSurfaceResponseItemDestinationChangePayloadValueThreeItemList(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_optional_query_releases_for_surface_response_item_destination_change_payload_value_three_item_list(self) -> Option<Vec<Option<QueryReleasesForSurfaceResponseItemDestinationChangePayloadValueThreeItem>>> {
        match self {
                    Self::OptionalQueryReleasesForSurfaceResponseItemDestinationChangePayloadValueThreeItemList(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_string_to_optional_query_releases_for_surface_response_item_destination_change_payload_value_four_value_map(&self) -> Option<&HashMap<String, Option<QueryReleasesForSurfaceResponseItemDestinationChangePayloadValueFourValue>>> {
        match self {
                    Self::StringToOptionalQueryReleasesForSurfaceResponseItemDestinationChangePayloadValueFourValueMap(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_string_to_optional_query_releases_for_surface_response_item_destination_change_payload_value_four_value_map(self) -> Option<HashMap<String, Option<QueryReleasesForSurfaceResponseItemDestinationChangePayloadValueFourValue>>> {
        match self {
                    Self::StringToOptionalQueryReleasesForSurfaceResponseItemDestinationChangePayloadValueFourValueMap(value) => Some(value),
                    _ => None,
                }
    }
}
