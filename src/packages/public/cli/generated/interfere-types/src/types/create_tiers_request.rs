pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateTiersRequest {
    #[serde(default)]
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shade: Option<CreateTiersRequestShade>,
}

impl CreateTiersRequest {
    pub fn builder() -> CreateTiersRequestBuilder {
        <CreateTiersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateTiersRequestBuilder {
    value: Option<String>,
    shade: Option<CreateTiersRequestShade>,
}

impl CreateTiersRequestBuilder {
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn shade(mut self, value: CreateTiersRequestShade) -> Self {
        self.shade = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateTiersRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`value`](CreateTiersRequestBuilder::value)
    pub fn build(self) -> Result<CreateTiersRequest, BuildError> {
        Ok(CreateTiersRequest {
            value: self.value.ok_or_else(|| BuildError::missing_field("value"))?,
            shade: self.shade,
        })
    }
}

