pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationTimelineEventsCreateCommentResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationTimelineEventsCreateCommentResponse {
    pub fn builder() -> MutationTimelineEventsCreateCommentResponseBuilder {
        <MutationTimelineEventsCreateCommentResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationTimelineEventsCreateCommentResponseBuilder {
    success: Option<bool>,
}

impl MutationTimelineEventsCreateCommentResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationTimelineEventsCreateCommentResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationTimelineEventsCreateCommentResponseBuilder::success)
    pub fn build(self) -> Result<MutationTimelineEventsCreateCommentResponse, BuildError> {
        Ok(MutationTimelineEventsCreateCommentResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
