pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SaveNetworkRulesRequestRulesItem {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub value: String,
}

impl SaveNetworkRulesRequestRulesItem {
    pub fn builder() -> SaveNetworkRulesRequestRulesItemBuilder {
        <SaveNetworkRulesRequestRulesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SaveNetworkRulesRequestRulesItemBuilder {
    id: Option<String>,
    label: Option<String>,
    value: Option<String>,
}

impl SaveNetworkRulesRequestRulesItemBuilder {
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

    /// Consumes the builder and constructs a [`SaveNetworkRulesRequestRulesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](SaveNetworkRulesRequestRulesItemBuilder::id)
    /// - [`label`](SaveNetworkRulesRequestRulesItemBuilder::label)
    /// - [`value`](SaveNetworkRulesRequestRulesItemBuilder::value)
    pub fn build(self) -> Result<SaveNetworkRulesRequestRulesItem, BuildError> {
        Ok(SaveNetworkRulesRequestRulesItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            label: self.label.ok_or_else(|| BuildError::missing_field("label"))?,
            value: self.value.ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
