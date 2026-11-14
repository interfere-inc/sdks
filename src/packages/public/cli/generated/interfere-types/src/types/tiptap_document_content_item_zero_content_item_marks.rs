pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TiptapDocumentContentItemZeroContentItemMarks {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marks: Option<Vec<TiptapMark>>,
    #[serde(default)]
    pub text: String,
    pub r#type: TiptapDocumentContentItemZeroContentItemMarksType,
}

impl TiptapDocumentContentItemZeroContentItemMarks {
    pub fn builder() -> TiptapDocumentContentItemZeroContentItemMarksBuilder {
        <TiptapDocumentContentItemZeroContentItemMarksBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TiptapDocumentContentItemZeroContentItemMarksBuilder {
    marks: Option<Vec<TiptapMark>>,
    text: Option<String>,
    r#type: Option<TiptapDocumentContentItemZeroContentItemMarksType>,
}

impl TiptapDocumentContentItemZeroContentItemMarksBuilder {
    pub fn marks(mut self, value: Vec<TiptapMark>) -> Self {
        self.marks = Some(value);
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: TiptapDocumentContentItemZeroContentItemMarksType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TiptapDocumentContentItemZeroContentItemMarks`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](TiptapDocumentContentItemZeroContentItemMarksBuilder::text)
    /// - [`r#type`](TiptapDocumentContentItemZeroContentItemMarksBuilder::r#type)
    pub fn build(self) -> Result<TiptapDocumentContentItemZeroContentItemMarks, BuildError> {
        Ok(TiptapDocumentContentItemZeroContentItemMarks {
            marks: self.marks,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
