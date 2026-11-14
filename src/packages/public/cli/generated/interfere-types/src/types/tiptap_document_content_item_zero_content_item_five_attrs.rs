pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TiptapDocumentContentItemZeroContentItemFiveAttrs {
    #[serde(rename = "releaseId")]
    #[serde(default)]
    pub release_id: String,
    #[serde(rename = "surfaceChangeId")]
    #[serde(default)]
    pub surface_change_id: String,
    pub snapshot: TiptapDocumentContentItemZeroContentItemFiveAttrsSnapshot,
}

impl TiptapDocumentContentItemZeroContentItemFiveAttrs {
    pub fn builder() -> TiptapDocumentContentItemZeroContentItemFiveAttrsBuilder {
        <TiptapDocumentContentItemZeroContentItemFiveAttrsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TiptapDocumentContentItemZeroContentItemFiveAttrsBuilder {
    release_id: Option<String>,
    surface_change_id: Option<String>,
    snapshot: Option<TiptapDocumentContentItemZeroContentItemFiveAttrsSnapshot>,
}

impl TiptapDocumentContentItemZeroContentItemFiveAttrsBuilder {
    pub fn release_id(mut self, value: impl Into<String>) -> Self {
        self.release_id = Some(value.into());
        self
    }

    pub fn surface_change_id(mut self, value: impl Into<String>) -> Self {
        self.surface_change_id = Some(value.into());
        self
    }

    pub fn snapshot(mut self, value: TiptapDocumentContentItemZeroContentItemFiveAttrsSnapshot) -> Self {
        self.snapshot = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TiptapDocumentContentItemZeroContentItemFiveAttrs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`release_id`](TiptapDocumentContentItemZeroContentItemFiveAttrsBuilder::release_id)
    /// - [`surface_change_id`](TiptapDocumentContentItemZeroContentItemFiveAttrsBuilder::surface_change_id)
    /// - [`snapshot`](TiptapDocumentContentItemZeroContentItemFiveAttrsBuilder::snapshot)
    pub fn build(self) -> Result<TiptapDocumentContentItemZeroContentItemFiveAttrs, BuildError> {
        Ok(TiptapDocumentContentItemZeroContentItemFiveAttrs {
            release_id: self.release_id.ok_or_else(|| BuildError::missing_field("release_id"))?,
            surface_change_id: self.surface_change_id.ok_or_else(|| BuildError::missing_field("surface_change_id"))?,
            snapshot: self.snapshot.ok_or_else(|| BuildError::missing_field("snapshot"))?,
        })
    }
}
