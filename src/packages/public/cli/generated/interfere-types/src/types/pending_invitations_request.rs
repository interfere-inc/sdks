pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PendingInvitationsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<HashMap<String, serde_json::Value>>,
}

impl PendingInvitationsRequest {
    pub fn builder() -> PendingInvitationsRequestBuilder {
        <PendingInvitationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PendingInvitationsRequestBuilder {
    args: Option<HashMap<String, serde_json::Value>>,
}

impl PendingInvitationsRequestBuilder {
    pub fn args(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PendingInvitationsRequest`].
    pub fn build(self) -> Result<PendingInvitationsRequest, BuildError> {
        Ok(PendingInvitationsRequest {
            args: self.args,
        })
    }
}

