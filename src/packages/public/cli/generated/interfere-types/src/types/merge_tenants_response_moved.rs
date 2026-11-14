pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MergeTenantsResponseMoved {
    #[serde(default)]
    pub domains: i64,
    #[serde(default)]
    pub users: i64,
}

impl MergeTenantsResponseMoved {
    pub fn builder() -> MergeTenantsResponseMovedBuilder {
        <MergeTenantsResponseMovedBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MergeTenantsResponseMovedBuilder {
    domains: Option<i64>,
    users: Option<i64>,
}

impl MergeTenantsResponseMovedBuilder {
    pub fn domains(mut self, value: i64) -> Self {
        self.domains = Some(value);
        self
    }

    pub fn users(mut self, value: i64) -> Self {
        self.users = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MergeTenantsResponseMoved`].
    /// This method will fail if any of the following fields are not set:
    /// - [`domains`](MergeTenantsResponseMovedBuilder::domains)
    /// - [`users`](MergeTenantsResponseMovedBuilder::users)
    pub fn build(self) -> Result<MergeTenantsResponseMoved, BuildError> {
        Ok(MergeTenantsResponseMoved {
            domains: self.domains.ok_or_else(|| BuildError::missing_field("domains"))?,
            users: self.users.ok_or_else(|| BuildError::missing_field("users"))?,
        })
    }
}
