pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UpdateCommentTimelineRequestArgs {
    #[serde(default)]
    pub id: String,
    pub doc: TiptapDocument,
}

impl UpdateCommentTimelineRequestArgs {
    pub fn builder() -> UpdateCommentTimelineRequestArgsBuilder {
        <UpdateCommentTimelineRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateCommentTimelineRequestArgsBuilder {
    id: Option<String>,
    doc: Option<TiptapDocument>,
}

impl UpdateCommentTimelineRequestArgsBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn doc(mut self, value: TiptapDocument) -> Self {
        self.doc = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateCommentTimelineRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](UpdateCommentTimelineRequestArgsBuilder::id)
    /// - [`doc`](UpdateCommentTimelineRequestArgsBuilder::doc)
    pub fn build(self) -> Result<UpdateCommentTimelineRequestArgs, BuildError> {
        Ok(UpdateCommentTimelineRequestArgs {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            doc: self.doc.ok_or_else(|| BuildError::missing_field("doc"))?,
        })
    }
}
