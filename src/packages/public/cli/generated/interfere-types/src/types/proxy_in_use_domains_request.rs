pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ProxyInUseDomainsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<HashMap<String, serde_json::Value>>,
}

impl ProxyInUseDomainsRequest {
    pub fn builder() -> ProxyInUseDomainsRequestBuilder {
        <ProxyInUseDomainsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProxyInUseDomainsRequestBuilder {
    args: Option<HashMap<String, serde_json::Value>>,
}

impl ProxyInUseDomainsRequestBuilder {
    pub fn args(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ProxyInUseDomainsRequest`].
    pub fn build(self) -> Result<ProxyInUseDomainsRequest, BuildError> {
        Ok(ProxyInUseDomainsRequest {
            args: self.args,
        })
    }
}

