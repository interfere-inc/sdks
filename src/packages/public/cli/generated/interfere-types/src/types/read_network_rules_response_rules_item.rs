pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReadNetworkRulesResponseRulesItem {
    /// Unique network rule identifier.
    #[serde(default)]
    pub id: String,
    /// Display label for the rule.
    #[serde(default)]
    pub label: String,
    /// Allowed network: a canonical CIDR range such as 203.0.113.0/24 or 2001:db8::/32, or an autonomous system number such as AS13335.
    #[serde(default)]
    pub value: String,
}

impl ReadNetworkRulesResponseRulesItem {
    pub fn builder() -> ReadNetworkRulesResponseRulesItemBuilder {
        <ReadNetworkRulesResponseRulesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReadNetworkRulesResponseRulesItemBuilder {
    id: Option<String>,
    label: Option<String>,
    value: Option<String>,
}

impl ReadNetworkRulesResponseRulesItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReadNetworkRulesResponseRulesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ReadNetworkRulesResponseRulesItemBuilder::id)
    /// - [`label`](ReadNetworkRulesResponseRulesItemBuilder::label)
    /// - [`value`](ReadNetworkRulesResponseRulesItemBuilder::value)
    pub fn build(self) -> Result<ReadNetworkRulesResponseRulesItem, BuildError> {
        Ok(ReadNetworkRulesResponseRulesItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            label: self.label.ok_or_else(|| BuildError::missing_field("label"))?,
            value: self.value.ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
