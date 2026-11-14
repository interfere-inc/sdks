pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListUsersRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<HashMap<String, serde_json::Value>>,
}

impl ListUsersRequest {
    pub fn builder() -> ListUsersRequestBuilder {
        <ListUsersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListUsersRequestBuilder {
    args: Option<HashMap<String, serde_json::Value>>,
}

impl ListUsersRequestBuilder {
    pub fn args(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListUsersRequest`].
    pub fn build(self) -> Result<ListUsersRequest, BuildError> {
        Ok(ListUsersRequest {
            args: self.args,
        })
    }
}

