pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Source map publication status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SourceMapPublicationStatus {
    #[serde(rename = "releaseSlug")]
    #[serde(default)]
    pub release_slug: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    #[serde(rename = "sessionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    pub state: SourceMapPublicationStatusState,
    #[serde(rename = "fileCount")]
    #[serde(default)]
    pub file_count: i64,
    #[serde(rename = "publishedFileCount")]
    #[serde(default)]
    pub published_file_count: i64,
    #[serde(rename = "batchDigest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_digest: Option<String>,
    #[serde(rename = "manifestReady")]
    #[serde(default)]
    pub manifest_ready: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnostics: Option<SourceMapPublicationStatusDiagnostics>,
}

impl SourceMapPublicationStatus {
    pub fn builder() -> SourceMapPublicationStatusBuilder {
        <SourceMapPublicationStatusBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SourceMapPublicationStatusBuilder {
    release_slug: Option<String>,
    environment: Option<String>,
    session_id: Option<String>,
    state: Option<SourceMapPublicationStatusState>,
    file_count: Option<i64>,
    published_file_count: Option<i64>,
    batch_digest: Option<String>,
    manifest_ready: Option<bool>,
    diagnostics: Option<SourceMapPublicationStatusDiagnostics>,
}

impl SourceMapPublicationStatusBuilder {
    pub fn release_slug(mut self, value: impl Into<String>) -> Self {
        self.release_slug = Some(value.into());
        self
    }

    pub fn environment(mut self, value: impl Into<String>) -> Self {
        self.environment = Some(value.into());
        self
    }

    pub fn session_id(mut self, value: impl Into<String>) -> Self {
        self.session_id = Some(value.into());
        self
    }

    pub fn state(mut self, value: SourceMapPublicationStatusState) -> Self {
        self.state = Some(value);
        self
    }

    pub fn file_count(mut self, value: i64) -> Self {
        self.file_count = Some(value);
        self
    }

    pub fn published_file_count(mut self, value: i64) -> Self {
        self.published_file_count = Some(value);
        self
    }

    pub fn batch_digest(mut self, value: impl Into<String>) -> Self {
        self.batch_digest = Some(value.into());
        self
    }

    pub fn manifest_ready(mut self, value: bool) -> Self {
        self.manifest_ready = Some(value);
        self
    }

    pub fn diagnostics(mut self, value: SourceMapPublicationStatusDiagnostics) -> Self {
        self.diagnostics = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SourceMapPublicationStatus`].
    /// This method will fail if any of the following fields are not set:
    /// - [`release_slug`](SourceMapPublicationStatusBuilder::release_slug)
    /// - [`state`](SourceMapPublicationStatusBuilder::state)
    /// - [`file_count`](SourceMapPublicationStatusBuilder::file_count)
    /// - [`published_file_count`](SourceMapPublicationStatusBuilder::published_file_count)
    /// - [`manifest_ready`](SourceMapPublicationStatusBuilder::manifest_ready)
    pub fn build(self) -> Result<SourceMapPublicationStatus, BuildError> {
        Ok(SourceMapPublicationStatus {
            release_slug: self.release_slug.ok_or_else(|| BuildError::missing_field("release_slug"))?,
            environment: self.environment,
            session_id: self.session_id,
            state: self.state.ok_or_else(|| BuildError::missing_field("state"))?,
            file_count: self.file_count.ok_or_else(|| BuildError::missing_field("file_count"))?,
            published_file_count: self.published_file_count.ok_or_else(|| BuildError::missing_field("published_file_count"))?,
            batch_digest: self.batch_digest,
            manifest_ready: self.manifest_ready.ok_or_else(|| BuildError::missing_field("manifest_ready"))?,
            diagnostics: self.diagnostics,
        })
    }
}
