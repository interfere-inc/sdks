pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SpanStackExceptionsItem {
    #[serde(rename = "eventIndex")]
    #[serde(default)]
    pub event_index: i64,
    #[serde(rename = "factId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fact_id: Option<String>,
    #[serde(rename = "exceptionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exception_type: Option<String>,
    #[serde(rename = "exceptionMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exception_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack: Option<String>,
    #[serde(default)]
    pub frames: Vec<SpanStackExceptionsItemFramesItem>,
}

impl SpanStackExceptionsItem {
    pub fn builder() -> SpanStackExceptionsItemBuilder {
        <SpanStackExceptionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SpanStackExceptionsItemBuilder {
    event_index: Option<i64>,
    fact_id: Option<String>,
    exception_type: Option<String>,
    exception_message: Option<String>,
    stack: Option<String>,
    frames: Option<Vec<SpanStackExceptionsItemFramesItem>>,
}

impl SpanStackExceptionsItemBuilder {
    pub fn event_index(mut self, value: i64) -> Self {
        self.event_index = Some(value);
        self
    }

    pub fn fact_id(mut self, value: impl Into<String>) -> Self {
        self.fact_id = Some(value.into());
        self
    }

    pub fn exception_type(mut self, value: impl Into<String>) -> Self {
        self.exception_type = Some(value.into());
        self
    }

    pub fn exception_message(mut self, value: impl Into<String>) -> Self {
        self.exception_message = Some(value.into());
        self
    }

    pub fn stack(mut self, value: impl Into<String>) -> Self {
        self.stack = Some(value.into());
        self
    }

    pub fn frames(mut self, value: Vec<SpanStackExceptionsItemFramesItem>) -> Self {
        self.frames = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SpanStackExceptionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`event_index`](SpanStackExceptionsItemBuilder::event_index)
    /// - [`frames`](SpanStackExceptionsItemBuilder::frames)
    pub fn build(self) -> Result<SpanStackExceptionsItem, BuildError> {
        Ok(SpanStackExceptionsItem {
            event_index: self.event_index.ok_or_else(|| BuildError::missing_field("event_index"))?,
            fact_id: self.fact_id,
            exception_type: self.exception_type,
            exception_message: self.exception_message,
            stack: self.stack,
            frames: self.frames.ok_or_else(|| BuildError::missing_field("frames"))?,
        })
    }
}
