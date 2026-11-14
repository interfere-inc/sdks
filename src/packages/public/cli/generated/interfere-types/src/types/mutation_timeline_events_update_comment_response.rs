pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationTimelineEventsUpdateCommentResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationTimelineEventsUpdateCommentResponse {
    pub fn builder() -> MutationTimelineEventsUpdateCommentResponseBuilder {
        <MutationTimelineEventsUpdateCommentResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationTimelineEventsUpdateCommentResponseBuilder {
    success: Option<bool>,
}

impl MutationTimelineEventsUpdateCommentResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationTimelineEventsUpdateCommentResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationTimelineEventsUpdateCommentResponseBuilder::success)
    pub fn build(self) -> Result<MutationTimelineEventsUpdateCommentResponse, BuildError> {
        Ok(MutationTimelineEventsUpdateCommentResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
