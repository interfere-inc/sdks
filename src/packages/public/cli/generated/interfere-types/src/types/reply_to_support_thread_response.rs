pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReplyToSupportThreadResponse {
    #[serde(default)]
    pub ok: String,
}

impl ReplyToSupportThreadResponse {
    pub fn builder() -> ReplyToSupportThreadResponseBuilder {
        <ReplyToSupportThreadResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReplyToSupportThreadResponseBuilder {
    ok: Option<String>,
}

impl ReplyToSupportThreadResponseBuilder {
    pub fn ok(mut self, value: impl Into<String>) -> Self {
        self.ok = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReplyToSupportThreadResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ok`](ReplyToSupportThreadResponseBuilder::ok)
    pub fn build(self) -> Result<ReplyToSupportThreadResponse, BuildError> {
        Ok(ReplyToSupportThreadResponse {
            ok: self.ok.ok_or_else(|| BuildError::missing_field("ok"))?,
        })
    }
}
