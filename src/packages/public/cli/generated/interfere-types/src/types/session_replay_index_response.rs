pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SessionReplayIndexResponse {
    #[serde(default)]
    pub chunks: Vec<SessionReplayIndexResponseChunksItem>,
    #[serde(default)]
    pub cursor: i64,
    #[serde(rename = "hasMore")]
    #[serde(default)]
    pub has_more: bool,
}

impl SessionReplayIndexResponse {
    pub fn builder() -> SessionReplayIndexResponseBuilder {
        <SessionReplayIndexResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SessionReplayIndexResponseBuilder {
    chunks: Option<Vec<SessionReplayIndexResponseChunksItem>>,
    cursor: Option<i64>,
    has_more: Option<bool>,
}

impl SessionReplayIndexResponseBuilder {
    pub fn chunks(mut self, value: Vec<SessionReplayIndexResponseChunksItem>) -> Self {
        self.chunks = Some(value);
        self
    }

    pub fn cursor(mut self, value: i64) -> Self {
        self.cursor = Some(value);
        self
    }

    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SessionReplayIndexResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`chunks`](SessionReplayIndexResponseBuilder::chunks)
    /// - [`cursor`](SessionReplayIndexResponseBuilder::cursor)
    /// - [`has_more`](SessionReplayIndexResponseBuilder::has_more)
    pub fn build(self) -> Result<SessionReplayIndexResponse, BuildError> {
        Ok(SessionReplayIndexResponse {
            chunks: self.chunks.ok_or_else(|| BuildError::missing_field("chunks"))?,
            cursor: self.cursor.ok_or_else(|| BuildError::missing_field("cursor"))?,
            has_more: self.has_more.ok_or_else(|| BuildError::missing_field("has_more"))?,
        })
    }
}
