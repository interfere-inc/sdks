pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SaveNetworkRulesResponse {
    #[serde(default)]
    pub success: bool,
}

impl SaveNetworkRulesResponse {
    pub fn builder() -> SaveNetworkRulesResponseBuilder {
        <SaveNetworkRulesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SaveNetworkRulesResponseBuilder {
    success: Option<bool>,
}

impl SaveNetworkRulesResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SaveNetworkRulesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](SaveNetworkRulesResponseBuilder::success)
    pub fn build(self) -> Result<SaveNetworkRulesResponse, BuildError> {
        Ok(SaveNetworkRulesResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
