pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LookupNetworkAsnResponse {
    #[serde(default)]
    pub success: bool,
}

impl LookupNetworkAsnResponse {
    pub fn builder() -> LookupNetworkAsnResponseBuilder {
        <LookupNetworkAsnResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LookupNetworkAsnResponseBuilder {
    success: Option<bool>,
}

impl LookupNetworkAsnResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LookupNetworkAsnResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](LookupNetworkAsnResponseBuilder::success)
    pub fn build(self) -> Result<LookupNetworkAsnResponse, BuildError> {
        Ok(LookupNetworkAsnResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
