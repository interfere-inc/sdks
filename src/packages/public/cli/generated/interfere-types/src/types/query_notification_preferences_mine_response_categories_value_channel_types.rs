pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryNotificationPreferencesMineResponseCategoriesValueChannelTypes {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_types: Option<HashMap<String, bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<QueryNotificationPreferencesMineResponseCategoriesValueChannelTypesConditionsItem>>,
}

impl QueryNotificationPreferencesMineResponseCategoriesValueChannelTypes {
    pub fn builder() -> QueryNotificationPreferencesMineResponseCategoriesValueChannelTypesBuilder {
        <QueryNotificationPreferencesMineResponseCategoriesValueChannelTypesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryNotificationPreferencesMineResponseCategoriesValueChannelTypesBuilder {
    channel_types: Option<HashMap<String, bool>>,
    conditions: Option<Vec<QueryNotificationPreferencesMineResponseCategoriesValueChannelTypesConditionsItem>>,
}

impl QueryNotificationPreferencesMineResponseCategoriesValueChannelTypesBuilder {
    pub fn channel_types(mut self, value: HashMap<String, bool>) -> Self {
        self.channel_types = Some(value);
        self
    }

    pub fn conditions(mut self, value: Vec<QueryNotificationPreferencesMineResponseCategoriesValueChannelTypesConditionsItem>) -> Self {
        self.conditions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryNotificationPreferencesMineResponseCategoriesValueChannelTypes`].
    pub fn build(self) -> Result<QueryNotificationPreferencesMineResponseCategoriesValueChannelTypes, BuildError> {
        Ok(QueryNotificationPreferencesMineResponseCategoriesValueChannelTypes {
            channel_types: self.channel_types,
            conditions: self.conditions,
        })
    }
}
