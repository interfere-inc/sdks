pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum QueryProblemsGetByIdResponseTimelineEventsItemReferencesItemSurfaceChangePayloadValue {
        String(String),

        Double(f64),

        Boolean(bool),

        OptionalQueryProblemsGetByIdResponseTimelineEventsItemReferencesItemSurfaceChangePayloadValueThreeItemList(Vec<Option<QueryProblemsGetByIdResponseTimelineEventsItemReferencesItemSurfaceChangePayloadValueThreeItem>>),

        StringToOptionalQueryProblemsGetByIdResponseTimelineEventsItemReferencesItemSurfaceChangePayloadValueFourValueMap(HashMap<String, Option<QueryProblemsGetByIdResponseTimelineEventsItemReferencesItemSurfaceChangePayloadValueFourValue>>),
}

impl QueryProblemsGetByIdResponseTimelineEventsItemReferencesItemSurfaceChangePayloadValue {
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_boolean(&self) -> bool {
        matches!(self, Self::Boolean(_))
    }

    pub fn is_optional_query_problems_get_by_id_response_timeline_events_item_references_item_surface_change_payload_value_three_item_list(&self) -> bool {
        matches!(self, Self::OptionalQueryProblemsGetByIdResponseTimelineEventsItemReferencesItemSurfaceChangePayloadValueThreeItemList(_))
    }

    pub fn is_string_to_optional_query_problems_get_by_id_response_timeline_events_item_references_item_surface_change_payload_value_four_value_map(&self) -> bool {
        matches!(self, Self::StringToOptionalQueryProblemsGetByIdResponseTimelineEventsItemReferencesItemSurfaceChangePayloadValueFourValueMap(_))
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

    pub fn as_optional_query_problems_get_by_id_response_timeline_events_item_references_item_surface_change_payload_value_three_item_list(&self) -> Option<&Vec<Option<QueryProblemsGetByIdResponseTimelineEventsItemReferencesItemSurfaceChangePayloadValueThreeItem>>> {
        match self {
                    Self::OptionalQueryProblemsGetByIdResponseTimelineEventsItemReferencesItemSurfaceChangePayloadValueThreeItemList(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_optional_query_problems_get_by_id_response_timeline_events_item_references_item_surface_change_payload_value_three_item_list(self) -> Option<Vec<Option<QueryProblemsGetByIdResponseTimelineEventsItemReferencesItemSurfaceChangePayloadValueThreeItem>>> {
        match self {
                    Self::OptionalQueryProblemsGetByIdResponseTimelineEventsItemReferencesItemSurfaceChangePayloadValueThreeItemList(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_string_to_optional_query_problems_get_by_id_response_timeline_events_item_references_item_surface_change_payload_value_four_value_map(&self) -> Option<&HashMap<String, Option<QueryProblemsGetByIdResponseTimelineEventsItemReferencesItemSurfaceChangePayloadValueFourValue>>> {
        match self {
                    Self::StringToOptionalQueryProblemsGetByIdResponseTimelineEventsItemReferencesItemSurfaceChangePayloadValueFourValueMap(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_string_to_optional_query_problems_get_by_id_response_timeline_events_item_references_item_surface_change_payload_value_four_value_map(self) -> Option<HashMap<String, Option<QueryProblemsGetByIdResponseTimelineEventsItemReferencesItemSurfaceChangePayloadValueFourValue>>> {
        match self {
                    Self::StringToOptionalQueryProblemsGetByIdResponseTimelineEventsItemReferencesItemSurfaceChangePayloadValueFourValueMap(value) => Some(value),
                    _ => None,
                }
    }
}
