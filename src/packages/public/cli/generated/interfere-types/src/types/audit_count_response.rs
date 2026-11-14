pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AuditCountResponse {
    #[serde(default)]
    pub count: i64,
    #[serde(default)]
    pub capped: bool,
}

impl AuditCountResponse {
    pub fn builder() -> AuditCountResponseBuilder {
        <AuditCountResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AuditCountResponseBuilder {
    count: Option<i64>,
    capped: Option<bool>,
}

impl AuditCountResponseBuilder {
    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn capped(mut self, value: bool) -> Self {
        self.capped = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AuditCountResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`count`](AuditCountResponseBuilder::count)
    /// - [`capped`](AuditCountResponseBuilder::capped)
    pub fn build(self) -> Result<AuditCountResponse, BuildError> {
        Ok(AuditCountResponse {
            count: self.count.ok_or_else(|| BuildError::missing_field("count"))?,
            capped: self.capped.ok_or_else(|| BuildError::missing_field("capped"))?,
        })
    }
}
