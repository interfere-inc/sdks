pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SoftDeleteCommentTimelineRequestArgs {
    #[serde(default)]
    pub id: String,
}

impl SoftDeleteCommentTimelineRequestArgs {
    pub fn builder() -> SoftDeleteCommentTimelineRequestArgsBuilder {
        <SoftDeleteCommentTimelineRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SoftDeleteCommentTimelineRequestArgsBuilder {
    id: Option<String>,
}

impl SoftDeleteCommentTimelineRequestArgsBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SoftDeleteCommentTimelineRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](SoftDeleteCommentTimelineRequestArgsBuilder::id)
    pub fn build(self) -> Result<SoftDeleteCommentTimelineRequestArgs, BuildError> {
        Ok(SoftDeleteCommentTimelineRequestArgs {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
