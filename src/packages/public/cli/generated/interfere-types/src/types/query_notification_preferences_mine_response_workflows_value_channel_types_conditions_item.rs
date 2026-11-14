pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryNotificationPreferencesMineResponseWorkflowsValueChannelTypesConditionsItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub argument: Option<String>,
    pub operator: QueryNotificationPreferencesMineResponseWorkflowsValueChannelTypesConditionsItemOperator,
    #[serde(default)]
    pub variable: String,
}

impl QueryNotificationPreferencesMineResponseWorkflowsValueChannelTypesConditionsItem {
    pub fn builder() -> QueryNotificationPreferencesMineResponseWorkflowsValueChannelTypesConditionsItemBuilder {
        <QueryNotificationPreferencesMineResponseWorkflowsValueChannelTypesConditionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryNotificationPreferencesMineResponseWorkflowsValueChannelTypesConditionsItemBuilder {
    argument: Option<String>,
    operator: Option<QueryNotificationPreferencesMineResponseWorkflowsValueChannelTypesConditionsItemOperator>,
    variable: Option<String>,
}

impl QueryNotificationPreferencesMineResponseWorkflowsValueChannelTypesConditionsItemBuilder {
    pub fn argument(mut self, value: impl Into<String>) -> Self {
        self.argument = Some(value.into());
        self
    }

    pub fn operator(mut self, value: QueryNotificationPreferencesMineResponseWorkflowsValueChannelTypesConditionsItemOperator) -> Self {
        self.operator = Some(value);
        self
    }

    pub fn variable(mut self, value: impl Into<String>) -> Self {
        self.variable = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryNotificationPreferencesMineResponseWorkflowsValueChannelTypesConditionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`operator`](QueryNotificationPreferencesMineResponseWorkflowsValueChannelTypesConditionsItemBuilder::operator)
    /// - [`variable`](QueryNotificationPreferencesMineResponseWorkflowsValueChannelTypesConditionsItemBuilder::variable)
    pub fn build(self) -> Result<QueryNotificationPreferencesMineResponseWorkflowsValueChannelTypesConditionsItem, BuildError> {
        Ok(QueryNotificationPreferencesMineResponseWorkflowsValueChannelTypesConditionsItem {
            argument: self.argument,
            operator: self.operator.ok_or_else(|| BuildError::missing_field("operator"))?,
            variable: self.variable.ok_or_else(|| BuildError::missing_field("variable"))?,
        })
    }
}
