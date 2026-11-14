pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SaveNetworkRulesRequest {
    #[serde(default)]
    pub rules: Vec<SaveNetworkRulesRequestRulesItem>,
}

impl SaveNetworkRulesRequest {
    pub fn builder() -> SaveNetworkRulesRequestBuilder {
        <SaveNetworkRulesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SaveNetworkRulesRequestBuilder {
    rules: Option<Vec<SaveNetworkRulesRequestRulesItem>>,
}

impl SaveNetworkRulesRequestBuilder {
    pub fn rules(mut self, value: Vec<SaveNetworkRulesRequestRulesItem>) -> Self {
        self.rules = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SaveNetworkRulesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rules`](SaveNetworkRulesRequestBuilder::rules)
    pub fn build(self) -> Result<SaveNetworkRulesRequest, BuildError> {
        Ok(SaveNetworkRulesRequest {
            rules: self.rules.ok_or_else(|| BuildError::missing_field("rules"))?,
        })
    }
}

