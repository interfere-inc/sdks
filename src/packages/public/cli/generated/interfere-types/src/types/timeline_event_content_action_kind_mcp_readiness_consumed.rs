pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum TimelineEventContentActionKindMcpReadinessConsumed {
        TimelineEventContentActionKindMcpReadinessConsumedConnectionId(TimelineEventContentActionKindMcpReadinessConsumedConnectionId),

        TimelineEventContentActionKindMcpReadinessConsumedIntegrationId(TimelineEventContentActionKindMcpReadinessConsumedIntegrationId),
}

impl TimelineEventContentActionKindMcpReadinessConsumed {
    pub fn is_timeline_event_content_action_kind_mcp_readiness_consumed_connection_id(&self) -> bool {
        matches!(self, Self::TimelineEventContentActionKindMcpReadinessConsumedConnectionId(_))
    }

    pub fn is_timeline_event_content_action_kind_mcp_readiness_consumed_integration_id(&self) -> bool {
        matches!(self, Self::TimelineEventContentActionKindMcpReadinessConsumedIntegrationId(_))
    }


    pub fn as_timeline_event_content_action_kind_mcp_readiness_consumed_connection_id(&self) -> Option<&TimelineEventContentActionKindMcpReadinessConsumedConnectionId> {
        match self {
                    Self::TimelineEventContentActionKindMcpReadinessConsumedConnectionId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_action_kind_mcp_readiness_consumed_connection_id(self) -> Option<TimelineEventContentActionKindMcpReadinessConsumedConnectionId> {
        match self {
                    Self::TimelineEventContentActionKindMcpReadinessConsumedConnectionId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_action_kind_mcp_readiness_consumed_integration_id(&self) -> Option<&TimelineEventContentActionKindMcpReadinessConsumedIntegrationId> {
        match self {
                    Self::TimelineEventContentActionKindMcpReadinessConsumedIntegrationId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_action_kind_mcp_readiness_consumed_integration_id(self) -> Option<TimelineEventContentActionKindMcpReadinessConsumedIntegrationId> {
        match self {
                    Self::TimelineEventContentActionKindMcpReadinessConsumedIntegrationId(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for TimelineEventContentActionKindMcpReadinessConsumed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TimelineEventContentActionKindMcpReadinessConsumedConnectionId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentActionKindMcpReadinessConsumedIntegrationId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
