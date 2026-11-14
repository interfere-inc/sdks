pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryNotificationPreferencesMineResponseCategoriesValueChannelTypesConditionsItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub argument: Option<String>,
    pub operator: QueryNotificationPreferencesMineResponseCategoriesValueChannelTypesConditionsItemOperator,
    #[serde(default)]
    pub variable: String,
}

impl QueryNotificationPreferencesMineResponseCategoriesValueChannelTypesConditionsItem {
    pub fn builder() -> QueryNotificationPreferencesMineResponseCategoriesValueChannelTypesConditionsItemBuilder {
        <QueryNotificationPreferencesMineResponseCategoriesValueChannelTypesConditionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryNotificationPreferencesMineResponseCategoriesValueChannelTypesConditionsItemBuilder {
    argument: Option<String>,
    operator: Option<QueryNotificationPreferencesMineResponseCategoriesValueChannelTypesConditionsItemOperator>,
    variable: Option<String>,
}

impl QueryNotificationPreferencesMineResponseCategoriesValueChannelTypesConditionsItemBuilder {
    pub fn argument(mut self, value: impl Into<String>) -> Self {
        self.argument = Some(value.into());
        self
    }

    pub fn operator(mut self, value: QueryNotificationPreferencesMineResponseCategoriesValueChannelTypesConditionsItemOperator) -> Self {
        self.operator = Some(value);
        self
    }

    pub fn variable(mut self, value: impl Into<String>) -> Self {
        self.variable = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryNotificationPreferencesMineResponseCategoriesValueChannelTypesConditionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`operator`](QueryNotificationPreferencesMineResponseCategoriesValueChannelTypesConditionsItemBuilder::operator)
    /// - [`variable`](QueryNotificationPreferencesMineResponseCategoriesValueChannelTypesConditionsItemBuilder::variable)
    pub fn build(self) -> Result<QueryNotificationPreferencesMineResponseCategoriesValueChannelTypesConditionsItem, BuildError> {
        Ok(QueryNotificationPreferencesMineResponseCategoriesValueChannelTypesConditionsItem {
            argument: self.argument,
            operator: self.operator.ok_or_else(|| BuildError::missing_field("operator"))?,
            variable: self.variable.ok_or_else(|| BuildError::missing_field("variable"))?,
        })
    }
}
