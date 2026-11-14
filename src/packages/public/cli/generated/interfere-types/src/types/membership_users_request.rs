pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MembershipUsersRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<HashMap<String, serde_json::Value>>,
}

impl MembershipUsersRequest {
    pub fn builder() -> MembershipUsersRequestBuilder {
        <MembershipUsersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MembershipUsersRequestBuilder {
    args: Option<HashMap<String, serde_json::Value>>,
}

impl MembershipUsersRequestBuilder {
    pub fn args(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MembershipUsersRequest`].
    pub fn build(self) -> Result<MembershipUsersRequest, BuildError> {
        Ok(MembershipUsersRequest {
            args: self.args,
        })
    }
}

