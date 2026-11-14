pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TiptapDocumentContentItemZeroContentItemOneAttrs {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub label: String,
    #[serde(rename = "mentionSuggestionChar")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mention_suggestion_char: Option<String>,
}

impl TiptapDocumentContentItemZeroContentItemOneAttrs {
    pub fn builder() -> TiptapDocumentContentItemZeroContentItemOneAttrsBuilder {
        <TiptapDocumentContentItemZeroContentItemOneAttrsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TiptapDocumentContentItemZeroContentItemOneAttrsBuilder {
    id: Option<String>,
    label: Option<String>,
    mention_suggestion_char: Option<String>,
}

impl TiptapDocumentContentItemZeroContentItemOneAttrsBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn mention_suggestion_char(mut self, value: impl Into<String>) -> Self {
        self.mention_suggestion_char = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TiptapDocumentContentItemZeroContentItemOneAttrs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TiptapDocumentContentItemZeroContentItemOneAttrsBuilder::id)
    /// - [`label`](TiptapDocumentContentItemZeroContentItemOneAttrsBuilder::label)
    pub fn build(self) -> Result<TiptapDocumentContentItemZeroContentItemOneAttrs, BuildError> {
        Ok(TiptapDocumentContentItemZeroContentItemOneAttrs {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            label: self.label.ok_or_else(|| BuildError::missing_field("label"))?,
            mention_suggestion_char: self.mention_suggestion_char,
        })
    }
}
