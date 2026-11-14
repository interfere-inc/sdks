pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum QueryNotificationPreferencesMineResponseWorkflowsValue {
        Boolean(bool),

        QueryNotificationPreferencesMineResponseWorkflowsValueChannelTypes(QueryNotificationPreferencesMineResponseWorkflowsValueChannelTypes),
}

impl QueryNotificationPreferencesMineResponseWorkflowsValue {
    pub fn is_boolean(&self) -> bool {
        matches!(self, Self::Boolean(_))
    }

    pub fn is_query_notification_preferences_mine_response_workflows_value_channel_types(&self) -> bool {
        matches!(self, Self::QueryNotificationPreferencesMineResponseWorkflowsValueChannelTypes(_))
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

    pub fn as_query_notification_preferences_mine_response_workflows_value_channel_types(&self) -> Option<&QueryNotificationPreferencesMineResponseWorkflowsValueChannelTypes> {
        match self {
                    Self::QueryNotificationPreferencesMineResponseWorkflowsValueChannelTypes(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_notification_preferences_mine_response_workflows_value_channel_types(self) -> Option<QueryNotificationPreferencesMineResponseWorkflowsValueChannelTypes> {
        match self {
                    Self::QueryNotificationPreferencesMineResponseWorkflowsValueChannelTypes(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryNotificationPreferencesMineResponseWorkflowsValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Boolean(value) => write!(f, "{}", value),
            Self::QueryNotificationPreferencesMineResponseWorkflowsValueChannelTypes(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
