pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteTrackingDomainResponse {
    #[serde(default)]
    pub ok: String,
}

impl DeleteTrackingDomainResponse {
    pub fn builder() -> DeleteTrackingDomainResponseBuilder {
        <DeleteTrackingDomainResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteTrackingDomainResponseBuilder {
    ok: Option<String>,
}

impl DeleteTrackingDomainResponseBuilder {
    pub fn ok(mut self, value: impl Into<String>) -> Self {
        self.ok = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteTrackingDomainResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ok`](DeleteTrackingDomainResponseBuilder::ok)
    pub fn build(self) -> Result<DeleteTrackingDomainResponse, BuildError> {
        Ok(DeleteTrackingDomainResponse {
            ok: self.ok.ok_or_else(|| BuildError::missing_field("ok"))?,
        })
    }
}
