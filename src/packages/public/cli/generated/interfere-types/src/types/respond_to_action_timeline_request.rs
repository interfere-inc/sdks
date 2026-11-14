pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RespondToActionTimelineRequest {
    #[serde(default)]
    pub args: RespondToActionTimelineRequestArgs,
}

impl RespondToActionTimelineRequest {
    pub fn builder() -> RespondToActionTimelineRequestBuilder {
        <RespondToActionTimelineRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RespondToActionTimelineRequestBuilder {
    args: Option<RespondToActionTimelineRequestArgs>,
}

impl RespondToActionTimelineRequestBuilder {
    pub fn args(mut self, value: RespondToActionTimelineRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RespondToActionTimelineRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](RespondToActionTimelineRequestBuilder::args)
    pub fn build(self) -> Result<RespondToActionTimelineRequest, BuildError> {
        Ok(RespondToActionTimelineRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

