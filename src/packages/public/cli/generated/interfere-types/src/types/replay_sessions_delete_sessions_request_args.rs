pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteSessionsRequestArgs {
    #[serde(rename = "replaySessionKey")]
    #[serde(default)]
    pub replay_session_key: String,
}

impl DeleteSessionsRequestArgs {
    pub fn builder() -> DeleteSessionsRequestArgsBuilder {
        <DeleteSessionsRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteSessionsRequestArgsBuilder {
    replay_session_key: Option<String>,
}

impl DeleteSessionsRequestArgsBuilder {
    pub fn replay_session_key(mut self, value: impl Into<String>) -> Self {
        self.replay_session_key = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteSessionsRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`replay_session_key`](DeleteSessionsRequestArgsBuilder::replay_session_key)
    pub fn build(self) -> Result<DeleteSessionsRequestArgs, BuildError> {
        Ok(DeleteSessionsRequestArgs {
            replay_session_key: self.replay_session_key.ok_or_else(|| BuildError::missing_field("replay_session_key"))?,
        })
    }
}
