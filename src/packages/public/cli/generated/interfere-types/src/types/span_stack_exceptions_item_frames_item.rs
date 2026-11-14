pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SpanStackExceptionsItemFramesItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub column: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#fn: Option<String>,
    #[serde(rename = "inApp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_app: Option<bool>,
    #[serde(rename = "ingestedId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingested_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub line: Option<f64>,
    #[serde(rename = "filePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[serde(rename = "fileReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_reference: Option<String>,
    #[serde(rename = "originalColumn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub original_column: Option<f64>,
    #[serde(rename = "originalLine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub original_line: Option<f64>,
    #[serde(rename = "originalName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved: Option<bool>,
    #[serde(rename = "sourceContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_context: Option<SpanStackExceptionsItemFramesItemSourceContext>,
    #[serde(rename = "rawFrameIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_frame_index: Option<i64>,
}

impl SpanStackExceptionsItemFramesItem {
    pub fn builder() -> SpanStackExceptionsItemFramesItemBuilder {
        <SpanStackExceptionsItemFramesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SpanStackExceptionsItemFramesItemBuilder {
    column: Option<f64>,
    file: Option<String>,
    r#fn: Option<String>,
    in_app: Option<bool>,
    ingested_id: Option<String>,
    line: Option<f64>,
    file_path: Option<String>,
    file_reference: Option<String>,
    original_column: Option<f64>,
    original_line: Option<f64>,
    original_name: Option<String>,
    resolved: Option<bool>,
    source_context: Option<SpanStackExceptionsItemFramesItemSourceContext>,
    raw_frame_index: Option<i64>,
}

impl SpanStackExceptionsItemFramesItemBuilder {
    pub fn column(mut self, value: f64) -> Self {
        self.column = Some(value);
        self
    }

    pub fn file(mut self, value: impl Into<String>) -> Self {
        self.file = Some(value.into());
        self
    }

    pub fn r#fn(mut self, value: impl Into<String>) -> Self {
        self.r#fn = Some(value.into());
        self
    }

    pub fn in_app(mut self, value: bool) -> Self {
        self.in_app = Some(value);
        self
    }

    pub fn ingested_id(mut self, value: impl Into<String>) -> Self {
        self.ingested_id = Some(value.into());
        self
    }

    pub fn line(mut self, value: f64) -> Self {
        self.line = Some(value);
        self
    }

    pub fn file_path(mut self, value: impl Into<String>) -> Self {
        self.file_path = Some(value.into());
        self
    }

    pub fn file_reference(mut self, value: impl Into<String>) -> Self {
        self.file_reference = Some(value.into());
        self
    }

    pub fn original_column(mut self, value: f64) -> Self {
        self.original_column = Some(value);
        self
    }

    pub fn original_line(mut self, value: f64) -> Self {
        self.original_line = Some(value);
        self
    }

    pub fn original_name(mut self, value: impl Into<String>) -> Self {
        self.original_name = Some(value.into());
        self
    }

    pub fn resolved(mut self, value: bool) -> Self {
        self.resolved = Some(value);
        self
    }

    pub fn source_context(mut self, value: SpanStackExceptionsItemFramesItemSourceContext) -> Self {
        self.source_context = Some(value);
        self
    }

    pub fn raw_frame_index(mut self, value: i64) -> Self {
        self.raw_frame_index = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SpanStackExceptionsItemFramesItem`].
    pub fn build(self) -> Result<SpanStackExceptionsItemFramesItem, BuildError> {
        Ok(SpanStackExceptionsItemFramesItem {
            column: self.column,
            file: self.file,
            r#fn: self.r#fn,
            in_app: self.in_app,
            ingested_id: self.ingested_id,
            line: self.line,
            file_path: self.file_path,
            file_reference: self.file_reference,
            original_column: self.original_column,
            original_line: self.original_line,
            original_name: self.original_name,
            resolved: self.resolved,
            source_context: self.source_context,
            raw_frame_index: self.raw_frame_index,
        })
    }
}
