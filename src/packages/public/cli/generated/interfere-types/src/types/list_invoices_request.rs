pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListInvoicesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<HashMap<String, serde_json::Value>>,
}

impl ListInvoicesRequest {
    pub fn builder() -> ListInvoicesRequestBuilder {
        <ListInvoicesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListInvoicesRequestBuilder {
    args: Option<HashMap<String, serde_json::Value>>,
}

impl ListInvoicesRequestBuilder {
    pub fn args(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListInvoicesRequest`].
    pub fn build(self) -> Result<ListInvoicesRequest, BuildError> {
        Ok(ListInvoicesRequest {
            args: self.args,
        })
    }
}

