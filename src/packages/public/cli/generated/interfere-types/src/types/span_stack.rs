pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SpanStack {
    #[serde(default)]
    pub exceptions: Vec<SpanStackExceptionsItem>,
    #[serde(default)]
    pub truncated: bool,
}

impl SpanStack {
    pub fn builder() -> SpanStackBuilder {
        <SpanStackBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SpanStackBuilder {
    exceptions: Option<Vec<SpanStackExceptionsItem>>,
    truncated: Option<bool>,
}

impl SpanStackBuilder {
    pub fn exceptions(mut self, value: Vec<SpanStackExceptionsItem>) -> Self {
        self.exceptions = Some(value);
        self
    }

    pub fn truncated(mut self, value: bool) -> Self {
        self.truncated = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SpanStack`].
    /// This method will fail if any of the following fields are not set:
    /// - [`exceptions`](SpanStackBuilder::exceptions)
    /// - [`truncated`](SpanStackBuilder::truncated)
    pub fn build(self) -> Result<SpanStack, BuildError> {
        Ok(SpanStack {
            exceptions: self.exceptions.ok_or_else(|| BuildError::missing_field("exceptions"))?,
            truncated: self.truncated.ok_or_else(|| BuildError::missing_field("truncated"))?,
        })
    }
}
