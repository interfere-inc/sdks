pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryNotificationPreferencesMineResponseWorkflowsValueChannelTypes {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_types: Option<HashMap<String, bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<QueryNotificationPreferencesMineResponseWorkflowsValueChannelTypesConditionsItem>>,
}

impl QueryNotificationPreferencesMineResponseWorkflowsValueChannelTypes {
    pub fn builder() -> QueryNotificationPreferencesMineResponseWorkflowsValueChannelTypesBuilder {
        <QueryNotificationPreferencesMineResponseWorkflowsValueChannelTypesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryNotificationPreferencesMineResponseWorkflowsValueChannelTypesBuilder {
    channel_types: Option<HashMap<String, bool>>,
    conditions: Option<Vec<QueryNotificationPreferencesMineResponseWorkflowsValueChannelTypesConditionsItem>>,
}

impl QueryNotificationPreferencesMineResponseWorkflowsValueChannelTypesBuilder {
    pub fn channel_types(mut self, value: HashMap<String, bool>) -> Self {
        self.channel_types = Some(value);
        self
    }

    pub fn conditions(mut self, value: Vec<QueryNotificationPreferencesMineResponseWorkflowsValueChannelTypesConditionsItem>) -> Self {
        self.conditions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryNotificationPreferencesMineResponseWorkflowsValueChannelTypes`].
    pub fn build(self) -> Result<QueryNotificationPreferencesMineResponseWorkflowsValueChannelTypes, BuildError> {
        Ok(QueryNotificationPreferencesMineResponseWorkflowsValueChannelTypes {
            channel_types: self.channel_types,
            conditions: self.conditions,
        })
    }
}
