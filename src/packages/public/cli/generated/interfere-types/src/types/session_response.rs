pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// SessionResponse
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SessionResponse {
    #[serde(default)]
    pub ok: bool,
}

impl SessionResponse {
    pub fn builder() -> SessionResponseBuilder {
        <SessionResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SessionResponseBuilder {
    ok: Option<bool>,
}

impl SessionResponseBuilder {
    pub fn ok(mut self, value: bool) -> Self {
        self.ok = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SessionResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ok`](SessionResponseBuilder::ok)
    pub fn build(self) -> Result<SessionResponse, BuildError> {
        Ok(SessionResponse {
            ok: self.ok.ok_or_else(|| BuildError::missing_field("ok"))?,
        })
    }
}
