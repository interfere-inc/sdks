pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpConnection {
        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpConnectionConnectionId(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpConnectionConnectionId),

        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpConnectionIntegrationId(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpConnectionIntegrationId),
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpConnection {
    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_action_kind_mcp_connection_connection_id(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpConnectionConnectionId(_))
    }

    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_action_kind_mcp_connection_integration_id(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpConnectionIntegrationId(_))
    }


    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_content_action_kind_mcp_connection_connection_id(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpConnectionConnectionId> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpConnectionConnectionId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_content_action_kind_mcp_connection_connection_id(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpConnectionConnectionId> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpConnectionConnectionId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_content_action_kind_mcp_connection_integration_id(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpConnectionIntegrationId> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpConnectionIntegrationId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_content_action_kind_mcp_connection_integration_id(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpConnectionIntegrationId> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpConnectionIntegrationId(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpConnection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpConnectionConnectionId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpConnectionIntegrationId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
