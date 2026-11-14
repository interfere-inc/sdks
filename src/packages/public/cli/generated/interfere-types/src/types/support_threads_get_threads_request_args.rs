pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetThreadsRequestArgs {
    #[serde(rename = "threadId")]
    #[serde(default)]
    pub thread_id: String,
}

impl GetThreadsRequestArgs {
    pub fn builder() -> GetThreadsRequestArgsBuilder {
        <GetThreadsRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetThreadsRequestArgsBuilder {
    thread_id: Option<String>,
}

impl GetThreadsRequestArgsBuilder {
    pub fn thread_id(mut self, value: impl Into<String>) -> Self {
        self.thread_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetThreadsRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`thread_id`](GetThreadsRequestArgsBuilder::thread_id)
    pub fn build(self) -> Result<GetThreadsRequestArgs, BuildError> {
        Ok(GetThreadsRequestArgs {
            thread_id: self.thread_id.ok_or_else(|| BuildError::missing_field("thread_id"))?,
        })
    }
}
