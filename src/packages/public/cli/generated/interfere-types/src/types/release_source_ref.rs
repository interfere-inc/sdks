pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// ReleaseSourceRef
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReleaseSourceRef {
    #[serde(default)]
    pub id: String,
}

impl ReleaseSourceRef {
    pub fn builder() -> ReleaseSourceRefBuilder {
        <ReleaseSourceRefBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReleaseSourceRefBuilder {
    id: Option<String>,
}

impl ReleaseSourceRefBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReleaseSourceRef`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ReleaseSourceRefBuilder::id)
    pub fn build(self) -> Result<ReleaseSourceRef, BuildError> {
        Ok(ReleaseSourceRef {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
