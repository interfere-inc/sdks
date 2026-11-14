pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SetServiceSourceResponse {
    #[serde(default)]
    pub ok: String,
}

impl SetServiceSourceResponse {
    pub fn builder() -> SetServiceSourceResponseBuilder {
        <SetServiceSourceResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetServiceSourceResponseBuilder {
    ok: Option<String>,
}

impl SetServiceSourceResponseBuilder {
    pub fn ok(mut self, value: impl Into<String>) -> Self {
        self.ok = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SetServiceSourceResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ok`](SetServiceSourceResponseBuilder::ok)
    pub fn build(self) -> Result<SetServiceSourceResponse, BuildError> {
        Ok(SetServiceSourceResponse {
            ok: self.ok.ok_or_else(|| BuildError::missing_field("ok"))?,
        })
    }
}
