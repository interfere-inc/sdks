pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Service the release belongs to, identified by its OpenTelemetry service.name and service.namespace.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReleaseServiceRef {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

impl ReleaseServiceRef {
    pub fn builder() -> ReleaseServiceRefBuilder {
        <ReleaseServiceRefBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReleaseServiceRefBuilder {
    id: Option<String>,
    name: Option<String>,
    namespace: Option<String>,
}

impl ReleaseServiceRefBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn namespace(mut self, value: impl Into<String>) -> Self {
        self.namespace = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReleaseServiceRef`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ReleaseServiceRefBuilder::id)
    /// - [`name`](ReleaseServiceRefBuilder::name)
    pub fn build(self) -> Result<ReleaseServiceRef, BuildError> {
        Ok(ReleaseServiceRef {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            namespace: self.namespace,
        })
    }
}
