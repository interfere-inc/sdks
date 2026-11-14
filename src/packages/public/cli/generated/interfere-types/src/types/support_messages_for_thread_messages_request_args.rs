pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ForThreadMessagesRequestArgs {
    #[serde(rename = "threadId")]
    #[serde(default)]
    pub thread_id: String,
}

impl ForThreadMessagesRequestArgs {
    pub fn builder() -> ForThreadMessagesRequestArgsBuilder {
        <ForThreadMessagesRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ForThreadMessagesRequestArgsBuilder {
    thread_id: Option<String>,
}

impl ForThreadMessagesRequestArgsBuilder {
    pub fn thread_id(mut self, value: impl Into<String>) -> Self {
        self.thread_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ForThreadMessagesRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`thread_id`](ForThreadMessagesRequestArgsBuilder::thread_id)
    pub fn build(self) -> Result<ForThreadMessagesRequestArgs, BuildError> {
        Ok(ForThreadMessagesRequestArgs {
            thread_id: self.thread_id.ok_or_else(|| BuildError::missing_field("thread_id"))?,
        })
    }
}
