pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MembershipsUsersRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<HashMap<String, serde_json::Value>>,
}

impl MembershipsUsersRequest {
    pub fn builder() -> MembershipsUsersRequestBuilder {
        <MembershipsUsersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MembershipsUsersRequestBuilder {
    args: Option<HashMap<String, serde_json::Value>>,
}

impl MembershipsUsersRequestBuilder {
    pub fn args(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MembershipsUsersRequest`].
    pub fn build(self) -> Result<MembershipsUsersRequest, BuildError> {
        Ok(MembershipsUsersRequest {
            args: self.args,
        })
    }
}

