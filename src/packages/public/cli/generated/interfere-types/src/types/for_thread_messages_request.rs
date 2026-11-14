pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ForThreadMessagesRequest {
    #[serde(default)]
    pub args: ForThreadMessagesRequestArgs,
}

impl ForThreadMessagesRequest {
    pub fn builder() -> ForThreadMessagesRequestBuilder {
        <ForThreadMessagesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ForThreadMessagesRequestBuilder {
    args: Option<ForThreadMessagesRequestArgs>,
}

impl ForThreadMessagesRequestBuilder {
    pub fn args(mut self, value: ForThreadMessagesRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ForThreadMessagesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](ForThreadMessagesRequestBuilder::args)
    pub fn build(self) -> Result<ForThreadMessagesRequest, BuildError> {
        Ok(ForThreadMessagesRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

