pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ArchiveServicesRequest {
    #[serde(default)]
    pub args: ArchiveServicesRequestArgs,
}

impl ArchiveServicesRequest {
    pub fn builder() -> ArchiveServicesRequestBuilder {
        <ArchiveServicesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ArchiveServicesRequestBuilder {
    args: Option<ArchiveServicesRequestArgs>,
}

impl ArchiveServicesRequestBuilder {
    pub fn args(mut self, value: ArchiveServicesRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ArchiveServicesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](ArchiveServicesRequestBuilder::args)
    pub fn build(self) -> Result<ArchiveServicesRequest, BuildError> {
        Ok(ArchiveServicesRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

