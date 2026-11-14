pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TiptapDocumentContentItemZeroContentItemSixAttrs {
    #[serde(rename = "observationId")]
    #[serde(default)]
    pub observation_id: String,
    #[serde(default)]
    pub snapshot: TiptapDocumentContentItemZeroContentItemSixAttrsSnapshot,
}

impl TiptapDocumentContentItemZeroContentItemSixAttrs {
    pub fn builder() -> TiptapDocumentContentItemZeroContentItemSixAttrsBuilder {
        <TiptapDocumentContentItemZeroContentItemSixAttrsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TiptapDocumentContentItemZeroContentItemSixAttrsBuilder {
    observation_id: Option<String>,
    snapshot: Option<TiptapDocumentContentItemZeroContentItemSixAttrsSnapshot>,
}

impl TiptapDocumentContentItemZeroContentItemSixAttrsBuilder {
    pub fn observation_id(mut self, value: impl Into<String>) -> Self {
        self.observation_id = Some(value.into());
        self
    }

    pub fn snapshot(mut self, value: TiptapDocumentContentItemZeroContentItemSixAttrsSnapshot) -> Self {
        self.snapshot = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TiptapDocumentContentItemZeroContentItemSixAttrs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`observation_id`](TiptapDocumentContentItemZeroContentItemSixAttrsBuilder::observation_id)
    /// - [`snapshot`](TiptapDocumentContentItemZeroContentItemSixAttrsBuilder::snapshot)
    pub fn build(self) -> Result<TiptapDocumentContentItemZeroContentItemSixAttrs, BuildError> {
        Ok(TiptapDocumentContentItemZeroContentItemSixAttrs {
            observation_id: self.observation_id.ok_or_else(|| BuildError::missing_field("observation_id"))?,
            snapshot: self.snapshot.ok_or_else(|| BuildError::missing_field("snapshot"))?,
        })
    }
}
