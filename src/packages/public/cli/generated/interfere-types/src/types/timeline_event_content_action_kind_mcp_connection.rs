pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum TimelineEventContentActionKindMcpConnection {
        TimelineEventContentActionKindMcpConnectionConnectionId(TimelineEventContentActionKindMcpConnectionConnectionId),

        TimelineEventContentActionKindMcpConnectionIntegrationId(TimelineEventContentActionKindMcpConnectionIntegrationId),
}

impl TimelineEventContentActionKindMcpConnection {
    pub fn is_timeline_event_content_action_kind_mcp_connection_connection_id(&self) -> bool {
        matches!(self, Self::TimelineEventContentActionKindMcpConnectionConnectionId(_))
    }

    pub fn is_timeline_event_content_action_kind_mcp_connection_integration_id(&self) -> bool {
        matches!(self, Self::TimelineEventContentActionKindMcpConnectionIntegrationId(_))
    }


    pub fn as_timeline_event_content_action_kind_mcp_connection_connection_id(&self) -> Option<&TimelineEventContentActionKindMcpConnectionConnectionId> {
        match self {
                    Self::TimelineEventContentActionKindMcpConnectionConnectionId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_action_kind_mcp_connection_connection_id(self) -> Option<TimelineEventContentActionKindMcpConnectionConnectionId> {
        match self {
                    Self::TimelineEventContentActionKindMcpConnectionConnectionId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_timeline_event_content_action_kind_mcp_connection_integration_id(&self) -> Option<&TimelineEventContentActionKindMcpConnectionIntegrationId> {
        match self {
                    Self::TimelineEventContentActionKindMcpConnectionIntegrationId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_timeline_event_content_action_kind_mcp_connection_integration_id(self) -> Option<TimelineEventContentActionKindMcpConnectionIntegrationId> {
        match self {
                    Self::TimelineEventContentActionKindMcpConnectionIntegrationId(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for TimelineEventContentActionKindMcpConnection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TimelineEventContentActionKindMcpConnectionConnectionId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TimelineEventContentActionKindMcpConnectionIntegrationId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
