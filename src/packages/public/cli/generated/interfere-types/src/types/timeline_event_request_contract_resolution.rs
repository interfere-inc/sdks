pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum TimelineEventRequestContractResolution {
        TimelineEventRequestContractResolutionZero(TimelineEventRequestContractResolutionZero),

        TimelineEventRequestContractResolutionReceipt(TimelineEventRequestContractResolutionReceipt),

        TimelineEventRequestContractResolutionTwo(TimelineEventRequestContractResolutionTwo),

        TimelineEventRequestContractResolutionAt(TimelineEventRequestContractResolutionAt),
}

impl TimelineEventRequestContractResolution {
    pub fn is_timeline_event_request_contract_resolution_zero(&self) -> bool {
        matches!(self, Self::TimelineEventRequestContractResolutionZero(_))
    }

    pub fn is_timeline_event_request_contract_resolution_receipt(&self) -> bool {
        matches!(self, Self::TimelineEventRequestContractResolutionReceipt(_))
    }

    pub fn is_timeline_event_request_contract_resolution_two(&self) -> bool {
        matches!(self, Self::TimelineEventRequestContractResolutionTwo(_))
    }

    pub fn is_timeline_event_request_contract_resolution_at(&self) -> bool {
        matches!(self, Self::TimelineEventRequestContractResolutionAt(_))
    }


    pub fn as_timeline_event_request_contract_resolution_zero(&self) -> Option<&TimelineEventRequestContractResolutionZero> {
        match self {
                    Self::TimelineEventRequestContractResolutionZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_request_contract_resolution_zero(self) -> Option<TimelineEventRequestContractResolutionZero> {
        match self {
                    Self::TimelineEventRequestContractResolutionZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_request_contract_resolution_receipt(&self) -> Option<&TimelineEventRequestContractResolutionReceipt> {
        match self {
                    Self::TimelineEventRequestContractResolutionReceipt(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_request_contract_resolution_receipt(self) -> Option<TimelineEventRequestContractResolutionReceipt> {
        match self {
                    Self::TimelineEventRequestContractResolutionReceipt(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_request_contract_resolution_two(&self) -> Option<&TimelineEventRequestContractResolutionTwo> {
        match self {
                    Self::TimelineEventRequestContractResolutionTwo(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_request_contract_resolution_two(self) -> Option<TimelineEventRequestContractResolutionTwo> {
        match self {
                    Self::TimelineEventRequestContractResolutionTwo(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_request_contract_resolution_at(&self) -> Option<&TimelineEventRequestContractResolutionAt> {
        match self {
                    Self::TimelineEventRequestContractResolutionAt(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_request_contract_resolution_at(self) -> Option<TimelineEventRequestContractResolutionAt> {
        match self {
                    Self::TimelineEventRequestContractResolutionAt(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for TimelineEventRequestContractResolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TimelineEventRequestContractResolutionZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventRequestContractResolutionReceipt(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventRequestContractResolutionTwo(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventRequestContractResolutionAt(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
