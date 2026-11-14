pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TiptapDocumentContentItemZeroContentItemFourAttrs {
    #[serde(rename = "surfaceChangeId")]
    #[serde(default)]
    pub surface_change_id: String,
    #[serde(default)]
    pub snapshot: TiptapDocumentContentItemZeroContentItemFourAttrsSnapshot,
}

impl TiptapDocumentContentItemZeroContentItemFourAttrs {
    pub fn builder() -> TiptapDocumentContentItemZeroContentItemFourAttrsBuilder {
        <TiptapDocumentContentItemZeroContentItemFourAttrsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TiptapDocumentContentItemZeroContentItemFourAttrsBuilder {
    surface_change_id: Option<String>,
    snapshot: Option<TiptapDocumentContentItemZeroContentItemFourAttrsSnapshot>,
}

impl TiptapDocumentContentItemZeroContentItemFourAttrsBuilder {
    pub fn surface_change_id(mut self, value: impl Into<String>) -> Self {
        self.surface_change_id = Some(value.into());
        self
    }

    pub fn snapshot(mut self, value: TiptapDocumentContentItemZeroContentItemFourAttrsSnapshot) -> Self {
        self.snapshot = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TiptapDocumentContentItemZeroContentItemFourAttrs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`surface_change_id`](TiptapDocumentContentItemZeroContentItemFourAttrsBuilder::surface_change_id)
    /// - [`snapshot`](TiptapDocumentContentItemZeroContentItemFourAttrsBuilder::snapshot)
    pub fn build(self) -> Result<TiptapDocumentContentItemZeroContentItemFourAttrs, BuildError> {
        Ok(TiptapDocumentContentItemZeroContentItemFourAttrs {
            surface_change_id: self.surface_change_id.ok_or_else(|| BuildError::missing_field("surface_change_id"))?,
            snapshot: self.snapshot.ok_or_else(|| BuildError::missing_field("snapshot"))?,
        })
    }
}
