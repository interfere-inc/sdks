pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MineInvitationsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<HashMap<String, serde_json::Value>>,
}

impl MineInvitationsRequest {
    pub fn builder() -> MineInvitationsRequestBuilder {
        <MineInvitationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MineInvitationsRequestBuilder {
    args: Option<HashMap<String, serde_json::Value>>,
}

impl MineInvitationsRequestBuilder {
    pub fn args(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MineInvitationsRequest`].
    pub fn build(self) -> Result<MineInvitationsRequest, BuildError> {
        Ok(MineInvitationsRequest {
            args: self.args,
        })
    }
}

