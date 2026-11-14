pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// CompleteSourceMapsResponse
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CompleteSourceMapsResponse {
    #[serde(rename = "fileCount")]
    #[serde(default)]
    pub file_count: i64,
    #[serde(default)]
    pub ok: bool,
}

impl CompleteSourceMapsResponse {
    pub fn builder() -> CompleteSourceMapsResponseBuilder {
        <CompleteSourceMapsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompleteSourceMapsResponseBuilder {
    file_count: Option<i64>,
    ok: Option<bool>,
}

impl CompleteSourceMapsResponseBuilder {
    pub fn file_count(mut self, value: i64) -> Self {
        self.file_count = Some(value);
        self
    }

    pub fn ok(mut self, value: bool) -> Self {
        self.ok = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CompleteSourceMapsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file_count`](CompleteSourceMapsResponseBuilder::file_count)
    /// - [`ok`](CompleteSourceMapsResponseBuilder::ok)
    pub fn build(self) -> Result<CompleteSourceMapsResponse, BuildError> {
        Ok(CompleteSourceMapsResponse {
            file_count: self.file_count.ok_or_else(|| BuildError::missing_field("file_count"))?,
            ok: self.ok.ok_or_else(|| BuildError::missing_field("ok"))?,
        })
    }
}
