pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListInvitationsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<HashMap<String, serde_json::Value>>,
}

impl ListInvitationsRequest {
    pub fn builder() -> ListInvitationsRequestBuilder {
        <ListInvitationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListInvitationsRequestBuilder {
    args: Option<HashMap<String, serde_json::Value>>,
}

impl ListInvitationsRequestBuilder {
    pub fn args(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListInvitationsRequest`].
    pub fn build(self) -> Result<ListInvitationsRequest, BuildError> {
        Ok(ListInvitationsRequest {
            args: self.args,
        })
    }
}

