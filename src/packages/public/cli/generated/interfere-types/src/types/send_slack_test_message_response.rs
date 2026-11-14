pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SendSlackTestMessageResponse {
    #[serde(default)]
    pub success: String,
}

impl SendSlackTestMessageResponse {
    pub fn builder() -> SendSlackTestMessageResponseBuilder {
        <SendSlackTestMessageResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SendSlackTestMessageResponseBuilder {
    success: Option<String>,
}

impl SendSlackTestMessageResponseBuilder {
    pub fn success(mut self, value: impl Into<String>) -> Self {
        self.success = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SendSlackTestMessageResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](SendSlackTestMessageResponseBuilder::success)
    pub fn build(self) -> Result<SendSlackTestMessageResponse, BuildError> {
        Ok(SendSlackTestMessageResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
