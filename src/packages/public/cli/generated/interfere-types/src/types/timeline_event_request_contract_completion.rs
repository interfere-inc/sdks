pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum TimelineEventRequestContractCompletion {
        TimelineEventRequestContractCompletionActionKind(TimelineEventRequestContractCompletionActionKind),

        TimelineEventRequestContractCompletionConcern(TimelineEventRequestContractCompletionConcern),

        TimelineEventRequestContractCompletionIntegrationName(TimelineEventRequestContractCompletionIntegrationName),

        TimelineEventRequestContractCompletionGapKind(TimelineEventRequestContractCompletionGapKind),
}

impl TimelineEventRequestContractCompletion {
    pub fn is_timeline_event_request_contract_completion_action_kind(&self) -> bool {
        matches!(self, Self::TimelineEventRequestContractCompletionActionKind(_))
    }

    pub fn is_timeline_event_request_contract_completion_concern(&self) -> bool {
        matches!(self, Self::TimelineEventRequestContractCompletionConcern(_))
    }

    pub fn is_timeline_event_request_contract_completion_integration_name(&self) -> bool {
        matches!(self, Self::TimelineEventRequestContractCompletionIntegrationName(_))
    }

    pub fn is_timeline_event_request_contract_completion_gap_kind(&self) -> bool {
        matches!(self, Self::TimelineEventRequestContractCompletionGapKind(_))
    }


    pub fn as_timeline_event_request_contract_completion_action_kind(&self) -> Option<&TimelineEventRequestContractCompletionActionKind> {
        match self {
                    Self::TimelineEventRequestContractCompletionActionKind(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_request_contract_completion_action_kind(self) -> Option<TimelineEventRequestContractCompletionActionKind> {
        match self {
                    Self::TimelineEventRequestContractCompletionActionKind(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_request_contract_completion_concern(&self) -> Option<&TimelineEventRequestContractCompletionConcern> {
        match self {
                    Self::TimelineEventRequestContractCompletionConcern(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_request_contract_completion_concern(self) -> Option<TimelineEventRequestContractCompletionConcern> {
        match self {
                    Self::TimelineEventRequestContractCompletionConcern(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_request_contract_completion_integration_name(&self) -> Option<&TimelineEventRequestContractCompletionIntegrationName> {
        match self {
                    Self::TimelineEventRequestContractCompletionIntegrationName(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_request_contract_completion_integration_name(self) -> Option<TimelineEventRequestContractCompletionIntegrationName> {
        match self {
                    Self::TimelineEventRequestContractCompletionIntegrationName(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_request_contract_completion_gap_kind(&self) -> Option<&TimelineEventRequestContractCompletionGapKind> {
        match self {
                    Self::TimelineEventRequestContractCompletionGapKind(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_request_contract_completion_gap_kind(self) -> Option<TimelineEventRequestContractCompletionGapKind> {
        match self {
                    Self::TimelineEventRequestContractCompletionGapKind(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for TimelineEventRequestContractCompletion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TimelineEventRequestContractCompletionActionKind(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventRequestContractCompletionConcern(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventRequestContractCompletionIntegrationName(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventRequestContractCompletionGapKind(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
