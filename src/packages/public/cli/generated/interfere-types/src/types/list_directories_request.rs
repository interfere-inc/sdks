pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListDirectoriesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<HashMap<String, serde_json::Value>>,
}

impl ListDirectoriesRequest {
    pub fn builder() -> ListDirectoriesRequestBuilder {
        <ListDirectoriesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListDirectoriesRequestBuilder {
    args: Option<HashMap<String, serde_json::Value>>,
}

impl ListDirectoriesRequestBuilder {
    pub fn args(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListDirectoriesRequest`].
    pub fn build(self) -> Result<ListDirectoriesRequest, BuildError> {
        Ok(ListDirectoriesRequest {
            args: self.args,
        })
    }
}

