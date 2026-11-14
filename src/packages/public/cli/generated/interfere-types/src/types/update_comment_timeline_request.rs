pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UpdateCommentTimelineRequest {
    pub args: UpdateCommentTimelineRequestArgs,
}

impl UpdateCommentTimelineRequest {
    pub fn builder() -> UpdateCommentTimelineRequestBuilder {
        <UpdateCommentTimelineRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateCommentTimelineRequestBuilder {
    args: Option<UpdateCommentTimelineRequestArgs>,
}

impl UpdateCommentTimelineRequestBuilder {
    pub fn args(mut self, value: UpdateCommentTimelineRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateCommentTimelineRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](UpdateCommentTimelineRequestBuilder::args)
    pub fn build(self) -> Result<UpdateCommentTimelineRequest, BuildError> {
        Ok(UpdateCommentTimelineRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

