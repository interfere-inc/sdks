pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListDomainsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<HashMap<String, serde_json::Value>>,
}

impl ListDomainsRequest {
    pub fn builder() -> ListDomainsRequestBuilder {
        <ListDomainsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListDomainsRequestBuilder {
    args: Option<HashMap<String, serde_json::Value>>,
}

impl ListDomainsRequestBuilder {
    pub fn args(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListDomainsRequest`].
    pub fn build(self) -> Result<ListDomainsRequest, BuildError> {
        Ok(ListDomainsRequest {
            args: self.args,
        })
    }
}

