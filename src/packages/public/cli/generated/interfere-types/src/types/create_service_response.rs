pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateServiceResponse {
    #[serde(default)]
    pub id: String,
}

impl CreateServiceResponse {
    pub fn builder() -> CreateServiceResponseBuilder {
        <CreateServiceResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateServiceResponseBuilder {
    id: Option<String>,
}

impl CreateServiceResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateServiceResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CreateServiceResponseBuilder::id)
    pub fn build(self) -> Result<CreateServiceResponse, BuildError> {
        Ok(CreateServiceResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
