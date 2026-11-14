pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TiptapDocumentContentItemZeroContentItemFourAttrsSnapshot {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub provider: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl TiptapDocumentContentItemZeroContentItemFourAttrsSnapshot {
    pub fn builder() -> TiptapDocumentContentItemZeroContentItemFourAttrsSnapshotBuilder {
        <TiptapDocumentContentItemZeroContentItemFourAttrsSnapshotBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TiptapDocumentContentItemZeroContentItemFourAttrsSnapshotBuilder {
    description: Option<String>,
    label: Option<String>,
    provider: Option<String>,
    url: Option<String>,
}

impl TiptapDocumentContentItemZeroContentItemFourAttrsSnapshotBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TiptapDocumentContentItemZeroContentItemFourAttrsSnapshot`].
    /// This method will fail if any of the following fields are not set:
    /// - [`label`](TiptapDocumentContentItemZeroContentItemFourAttrsSnapshotBuilder::label)
    /// - [`provider`](TiptapDocumentContentItemZeroContentItemFourAttrsSnapshotBuilder::provider)
    pub fn build(self) -> Result<TiptapDocumentContentItemZeroContentItemFourAttrsSnapshot, BuildError> {
        Ok(TiptapDocumentContentItemZeroContentItemFourAttrsSnapshot {
            description: self.description,
            label: self.label.ok_or_else(|| BuildError::missing_field("label"))?,
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
            url: self.url,
        })
    }
}
