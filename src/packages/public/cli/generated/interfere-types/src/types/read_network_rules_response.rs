pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReadNetworkRulesResponse {
    /// Networks allowed to reach the workspace dashboard and API. An empty list means no restriction.
    #[serde(default)]
    pub rules: Vec<ReadNetworkRulesResponseRulesItem>,
}

impl ReadNetworkRulesResponse {
    pub fn builder() -> ReadNetworkRulesResponseBuilder {
        <ReadNetworkRulesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReadNetworkRulesResponseBuilder {
    rules: Option<Vec<ReadNetworkRulesResponseRulesItem>>,
}

impl ReadNetworkRulesResponseBuilder {
    pub fn rules(mut self, value: Vec<ReadNetworkRulesResponseRulesItem>) -> Self {
        self.rules = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReadNetworkRulesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rules`](ReadNetworkRulesResponseBuilder::rules)
    pub fn build(self) -> Result<ReadNetworkRulesResponse, BuildError> {
        Ok(ReadNetworkRulesResponse {
            rules: self.rules.ok_or_else(|| BuildError::missing_field("rules"))?,
        })
    }
}
