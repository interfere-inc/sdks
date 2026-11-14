pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListSurfacesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<HashMap<String, serde_json::Value>>,
}

impl ListSurfacesRequest {
    pub fn builder() -> ListSurfacesRequestBuilder {
        <ListSurfacesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListSurfacesRequestBuilder {
    args: Option<HashMap<String, serde_json::Value>>,
}

impl ListSurfacesRequestBuilder {
    pub fn args(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListSurfacesRequest`].
    pub fn build(self) -> Result<ListSurfacesRequest, BuildError> {
        Ok(ListSurfacesRequest {
            args: self.args,
        })
    }
}

