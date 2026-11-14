pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationTimelineEventsSoftDeleteCommentResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationTimelineEventsSoftDeleteCommentResponse {
    pub fn builder() -> MutationTimelineEventsSoftDeleteCommentResponseBuilder {
        <MutationTimelineEventsSoftDeleteCommentResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationTimelineEventsSoftDeleteCommentResponseBuilder {
    success: Option<bool>,
}

impl MutationTimelineEventsSoftDeleteCommentResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationTimelineEventsSoftDeleteCommentResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationTimelineEventsSoftDeleteCommentResponseBuilder::success)
    pub fn build(self) -> Result<MutationTimelineEventsSoftDeleteCommentResponse, BuildError> {
        Ok(MutationTimelineEventsSoftDeleteCommentResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
