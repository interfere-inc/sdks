pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateCommentTimelineRequestArgs {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "problemId")]
    #[serde(default)]
    pub problem_id: String,
    #[serde(rename = "parentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    pub doc: TiptapDocument,
}

impl CreateCommentTimelineRequestArgs {
    pub fn builder() -> CreateCommentTimelineRequestArgsBuilder {
        <CreateCommentTimelineRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateCommentTimelineRequestArgsBuilder {
    id: Option<String>,
    problem_id: Option<String>,
    parent_id: Option<String>,
    doc: Option<TiptapDocument>,
}

impl CreateCommentTimelineRequestArgsBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn problem_id(mut self, value: impl Into<String>) -> Self {
        self.problem_id = Some(value.into());
        self
    }

    pub fn parent_id(mut self, value: impl Into<String>) -> Self {
        self.parent_id = Some(value.into());
        self
    }

    pub fn doc(mut self, value: TiptapDocument) -> Self {
        self.doc = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateCommentTimelineRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CreateCommentTimelineRequestArgsBuilder::id)
    /// - [`problem_id`](CreateCommentTimelineRequestArgsBuilder::problem_id)
    /// - [`doc`](CreateCommentTimelineRequestArgsBuilder::doc)
    pub fn build(self) -> Result<CreateCommentTimelineRequestArgs, BuildError> {
        Ok(CreateCommentTimelineRequestArgs {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            problem_id: self.problem_id.ok_or_else(|| BuildError::missing_field("problem_id"))?,
            parent_id: self.parent_id,
            doc: self.doc.ok_or_else(|| BuildError::missing_field("doc"))?,
        })
    }
}
