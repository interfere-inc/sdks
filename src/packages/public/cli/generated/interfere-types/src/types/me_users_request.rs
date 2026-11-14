pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MeUsersRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<HashMap<String, serde_json::Value>>,
}

impl MeUsersRequest {
    pub fn builder() -> MeUsersRequestBuilder {
        <MeUsersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MeUsersRequestBuilder {
    args: Option<HashMap<String, serde_json::Value>>,
}

impl MeUsersRequestBuilder {
    pub fn args(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MeUsersRequest`].
    pub fn build(self) -> Result<MeUsersRequest, BuildError> {
        Ok(MeUsersRequest {
            args: self.args,
        })
    }
}

