pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SpanStackExceptionsItemFramesItemSourceContext {
    #[serde(rename = "startLine")]
    #[serde(default)]
    pub start_line: i64,
    #[serde(default)]
    pub code: String,
}

impl SpanStackExceptionsItemFramesItemSourceContext {
    pub fn builder() -> SpanStackExceptionsItemFramesItemSourceContextBuilder {
        <SpanStackExceptionsItemFramesItemSourceContextBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SpanStackExceptionsItemFramesItemSourceContextBuilder {
    start_line: Option<i64>,
    code: Option<String>,
}

impl SpanStackExceptionsItemFramesItemSourceContextBuilder {
    pub fn start_line(mut self, value: i64) -> Self {
        self.start_line = Some(value);
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SpanStackExceptionsItemFramesItemSourceContext`].
    /// This method will fail if any of the following fields are not set:
    /// - [`start_line`](SpanStackExceptionsItemFramesItemSourceContextBuilder::start_line)
    /// - [`code`](SpanStackExceptionsItemFramesItemSourceContextBuilder::code)
    pub fn build(self) -> Result<SpanStackExceptionsItemFramesItemSourceContext, BuildError> {
        Ok(SpanStackExceptionsItemFramesItemSourceContext {
            start_line: self.start_line.ok_or_else(|| BuildError::missing_field("start_line"))?,
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
        })
    }
}
