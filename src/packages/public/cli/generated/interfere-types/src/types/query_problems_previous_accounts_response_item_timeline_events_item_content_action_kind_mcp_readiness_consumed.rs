pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpReadinessConsumed {
        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpReadinessConsumedConnectionId(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpReadinessConsumedConnectionId),

        QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpReadinessConsumedIntegrationId(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpReadinessConsumedIntegrationId),
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpReadinessConsumed {
    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_action_kind_mcp_readiness_consumed_connection_id(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpReadinessConsumedConnectionId(_))
    }

    pub fn is_query_problems_previous_accounts_response_item_timeline_events_item_content_action_kind_mcp_readiness_consumed_integration_id(&self) -> bool {
        matches!(self, Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpReadinessConsumedIntegrationId(_))
    }


    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_content_action_kind_mcp_readiness_consumed_connection_id(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpReadinessConsumedConnectionId> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpReadinessConsumedConnectionId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_content_action_kind_mcp_readiness_consumed_connection_id(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpReadinessConsumedConnectionId> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpReadinessConsumedConnectionId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_query_problems_previous_accounts_response_item_timeline_events_item_content_action_kind_mcp_readiness_consumed_integration_id(&self) -> Option<&QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpReadinessConsumedIntegrationId> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpReadinessConsumedIntegrationId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_problems_previous_accounts_response_item_timeline_events_item_content_action_kind_mcp_readiness_consumed_integration_id(self) -> Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpReadinessConsumedIntegrationId> {
        match self {
                    Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpReadinessConsumedIntegrationId(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpReadinessConsumed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpReadinessConsumedConnectionId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpReadinessConsumedIntegrationId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
