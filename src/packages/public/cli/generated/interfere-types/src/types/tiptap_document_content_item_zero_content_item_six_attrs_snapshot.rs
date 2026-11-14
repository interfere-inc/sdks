pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TiptapDocumentContentItemZeroContentItemSixAttrsSnapshot {
    #[serde(default)]
    pub sequence: i64,
    #[serde(rename = "textPreview")]
    #[serde(default)]
    pub text_preview: String,
}

impl TiptapDocumentContentItemZeroContentItemSixAttrsSnapshot {
    pub fn builder() -> TiptapDocumentContentItemZeroContentItemSixAttrsSnapshotBuilder {
        <TiptapDocumentContentItemZeroContentItemSixAttrsSnapshotBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TiptapDocumentContentItemZeroContentItemSixAttrsSnapshotBuilder {
    sequence: Option<i64>,
    text_preview: Option<String>,
}

impl TiptapDocumentContentItemZeroContentItemSixAttrsSnapshotBuilder {
    pub fn sequence(mut self, value: i64) -> Self {
        self.sequence = Some(value);
        self
    }

    pub fn text_preview(mut self, value: impl Into<String>) -> Self {
        self.text_preview = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TiptapDocumentContentItemZeroContentItemSixAttrsSnapshot`].
    /// This method will fail if any of the following fields are not set:
    /// - [`sequence`](TiptapDocumentContentItemZeroContentItemSixAttrsSnapshotBuilder::sequence)
    /// - [`text_preview`](TiptapDocumentContentItemZeroContentItemSixAttrsSnapshotBuilder::text_preview)
    pub fn build(self) -> Result<TiptapDocumentContentItemZeroContentItemSixAttrsSnapshot, BuildError> {
        Ok(TiptapDocumentContentItemZeroContentItemSixAttrsSnapshot {
            sequence: self.sequence.ok_or_else(|| BuildError::missing_field("sequence"))?,
            text_preview: self.text_preview.ok_or_else(|| BuildError::missing_field("text_preview"))?,
        })
    }
}
