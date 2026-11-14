pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TiptapDocumentContentItemZeroContentItemFiveAttrsSnapshot {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor: Option<String>,
    #[serde(rename = "deployedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub deployed_at: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub provider: String,
    #[serde(rename = "sourceFileCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_file_count: Option<i64>,
    #[serde(rename = "sourceMapCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_map_count: Option<i64>,
    pub status: TiptapDocumentContentItemZeroContentItemFiveAttrsSnapshotStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl TiptapDocumentContentItemZeroContentItemFiveAttrsSnapshot {
    pub fn builder() -> TiptapDocumentContentItemZeroContentItemFiveAttrsSnapshotBuilder {
        <TiptapDocumentContentItemZeroContentItemFiveAttrsSnapshotBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TiptapDocumentContentItemZeroContentItemFiveAttrsSnapshotBuilder {
    actor: Option<String>,
    deployed_at: Option<DateTime<FixedOffset>>,
    environment: Option<String>,
    label: Option<String>,
    provider: Option<String>,
    source_file_count: Option<i64>,
    source_map_count: Option<i64>,
    status: Option<TiptapDocumentContentItemZeroContentItemFiveAttrsSnapshotStatus>,
    url: Option<String>,
}

impl TiptapDocumentContentItemZeroContentItemFiveAttrsSnapshotBuilder {
    pub fn actor(mut self, value: impl Into<String>) -> Self {
        self.actor = Some(value.into());
        self
    }

    pub fn deployed_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.deployed_at = Some(value);
        self
    }

    pub fn environment(mut self, value: impl Into<String>) -> Self {
        self.environment = Some(value.into());
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

    pub fn source_file_count(mut self, value: i64) -> Self {
        self.source_file_count = Some(value);
        self
    }

    pub fn source_map_count(mut self, value: i64) -> Self {
        self.source_map_count = Some(value);
        self
    }

    pub fn status(mut self, value: TiptapDocumentContentItemZeroContentItemFiveAttrsSnapshotStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TiptapDocumentContentItemZeroContentItemFiveAttrsSnapshot`].
    /// This method will fail if any of the following fields are not set:
    /// - [`label`](TiptapDocumentContentItemZeroContentItemFiveAttrsSnapshotBuilder::label)
    /// - [`provider`](TiptapDocumentContentItemZeroContentItemFiveAttrsSnapshotBuilder::provider)
    /// - [`status`](TiptapDocumentContentItemZeroContentItemFiveAttrsSnapshotBuilder::status)
    pub fn build(self) -> Result<TiptapDocumentContentItemZeroContentItemFiveAttrsSnapshot, BuildError> {
        Ok(TiptapDocumentContentItemZeroContentItemFiveAttrsSnapshot {
            actor: self.actor,
            deployed_at: self.deployed_at,
            environment: self.environment,
            label: self.label.ok_or_else(|| BuildError::missing_field("label"))?,
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
            source_file_count: self.source_file_count,
            source_map_count: self.source_map_count,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            url: self.url,
        })
    }
}
