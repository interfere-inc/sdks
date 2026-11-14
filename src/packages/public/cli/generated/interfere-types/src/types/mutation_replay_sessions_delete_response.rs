pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationReplaySessionsDeleteResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationReplaySessionsDeleteResponse {
    pub fn builder() -> MutationReplaySessionsDeleteResponseBuilder {
        <MutationReplaySessionsDeleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationReplaySessionsDeleteResponseBuilder {
    success: Option<bool>,
}

impl MutationReplaySessionsDeleteResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationReplaySessionsDeleteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationReplaySessionsDeleteResponseBuilder::success)
    pub fn build(self) -> Result<MutationReplaySessionsDeleteResponse, BuildError> {
        Ok(MutationReplaySessionsDeleteResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
