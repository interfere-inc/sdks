pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SetTypeServicesRequestArgs {
    #[serde(rename = "serviceId")]
    #[serde(default)]
    pub service_id: String,
    pub r#type: SetTypeServicesRequestArgsType,
}

impl SetTypeServicesRequestArgs {
    pub fn builder() -> SetTypeServicesRequestArgsBuilder {
        <SetTypeServicesRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetTypeServicesRequestArgsBuilder {
    service_id: Option<String>,
    r#type: Option<SetTypeServicesRequestArgsType>,
}

impl SetTypeServicesRequestArgsBuilder {
    pub fn service_id(mut self, value: impl Into<String>) -> Self {
        self.service_id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: SetTypeServicesRequestArgsType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SetTypeServicesRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`service_id`](SetTypeServicesRequestArgsBuilder::service_id)
    /// - [`r#type`](SetTypeServicesRequestArgsBuilder::r#type)
    pub fn build(self) -> Result<SetTypeServicesRequestArgs, BuildError> {
        Ok(SetTypeServicesRequestArgs {
            service_id: self.service_id.ok_or_else(|| BuildError::missing_field("service_id"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
