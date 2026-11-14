pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateCommentTimelineRequest {
    pub args: CreateCommentTimelineRequestArgs,
}

impl CreateCommentTimelineRequest {
    pub fn builder() -> CreateCommentTimelineRequestBuilder {
        <CreateCommentTimelineRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateCommentTimelineRequestBuilder {
    args: Option<CreateCommentTimelineRequestArgs>,
}

impl CreateCommentTimelineRequestBuilder {
    pub fn args(mut self, value: CreateCommentTimelineRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateCommentTimelineRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](CreateCommentTimelineRequestBuilder::args)
    pub fn build(self) -> Result<CreateCommentTimelineRequest, BuildError> {
        Ok(CreateCommentTimelineRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

