pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// ReleaseService
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReleaseService {
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

impl ReleaseService {
    pub fn builder() -> ReleaseServiceBuilder {
        <ReleaseServiceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReleaseServiceBuilder {
    name: Option<String>,
    namespace: Option<String>,
}

impl ReleaseServiceBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn namespace(mut self, value: impl Into<String>) -> Self {
        self.namespace = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReleaseService`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ReleaseServiceBuilder::name)
    pub fn build(self) -> Result<ReleaseService, BuildError> {
        Ok(ReleaseService {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            namespace: self.namespace,
        })
    }
}
