pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TiptapDocumentContentItemZeroContentItemSevenAttrs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<i64>,
    #[serde(default)]
    pub path: String,
    #[serde(default)]
    pub r#ref: String,
    #[serde(default)]
    pub repository: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl TiptapDocumentContentItemZeroContentItemSevenAttrs {
    pub fn builder() -> TiptapDocumentContentItemZeroContentItemSevenAttrsBuilder {
        <TiptapDocumentContentItemZeroContentItemSevenAttrsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TiptapDocumentContentItemZeroContentItemSevenAttrsBuilder {
    line: Option<i64>,
    path: Option<String>,
    r#ref: Option<String>,
    repository: Option<String>,
    url: Option<String>,
}

impl TiptapDocumentContentItemZeroContentItemSevenAttrsBuilder {
    pub fn line(mut self, value: i64) -> Self {
        self.line = Some(value);
        self
    }

    pub fn path(mut self, value: impl Into<String>) -> Self {
        self.path = Some(value.into());
        self
    }

    pub fn r#ref(mut self, value: impl Into<String>) -> Self {
        self.r#ref = Some(value.into());
        self
    }

    pub fn repository(mut self, value: impl Into<String>) -> Self {
        self.repository = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TiptapDocumentContentItemZeroContentItemSevenAttrs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`path`](TiptapDocumentContentItemZeroContentItemSevenAttrsBuilder::path)
    /// - [`r#ref`](TiptapDocumentContentItemZeroContentItemSevenAttrsBuilder::r#ref)
    /// - [`repository`](TiptapDocumentContentItemZeroContentItemSevenAttrsBuilder::repository)
    pub fn build(self) -> Result<TiptapDocumentContentItemZeroContentItemSevenAttrs, BuildError> {
        Ok(TiptapDocumentContentItemZeroContentItemSevenAttrs {
            line: self.line,
            path: self.path.ok_or_else(|| BuildError::missing_field("path"))?,
            r#ref: self.r#ref.ok_or_else(|| BuildError::missing_field("r#ref"))?,
            repository: self.repository.ok_or_else(|| BuildError::missing_field("repository"))?,
            url: self.url,
        })
    }
}
