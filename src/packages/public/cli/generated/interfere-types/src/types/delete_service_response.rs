pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteServiceResponse {
    #[serde(default)]
    pub ok: String,
}

impl DeleteServiceResponse {
    pub fn builder() -> DeleteServiceResponseBuilder {
        <DeleteServiceResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteServiceResponseBuilder {
    ok: Option<String>,
}

impl DeleteServiceResponseBuilder {
    pub fn ok(mut self, value: impl Into<String>) -> Self {
        self.ok = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteServiceResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ok`](DeleteServiceResponseBuilder::ok)
    pub fn build(self) -> Result<DeleteServiceResponse, BuildError> {
        Ok(DeleteServiceResponse {
            ok: self.ok.ok_or_else(|| BuildError::missing_field("ok"))?,
        })
    }
}
