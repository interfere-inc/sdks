pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SoftDeleteCommentTimelineRequest {
    #[serde(default)]
    pub args: SoftDeleteCommentTimelineRequestArgs,
}

impl SoftDeleteCommentTimelineRequest {
    pub fn builder() -> SoftDeleteCommentTimelineRequestBuilder {
        <SoftDeleteCommentTimelineRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SoftDeleteCommentTimelineRequestBuilder {
    args: Option<SoftDeleteCommentTimelineRequestArgs>,
}

impl SoftDeleteCommentTimelineRequestBuilder {
    pub fn args(mut self, value: SoftDeleteCommentTimelineRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SoftDeleteCommentTimelineRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](SoftDeleteCommentTimelineRequestBuilder::args)
    pub fn build(self) -> Result<SoftDeleteCommentTimelineRequest, BuildError> {
        Ok(SoftDeleteCommentTimelineRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

