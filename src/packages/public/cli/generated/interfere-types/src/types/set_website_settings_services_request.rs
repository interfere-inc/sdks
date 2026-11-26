pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SetWebsiteSettingsServicesRequest {
    pub args: SetWebsiteSettingsServicesRequestArgs,
}

impl SetWebsiteSettingsServicesRequest {
    pub fn builder() -> SetWebsiteSettingsServicesRequestBuilder {
        <SetWebsiteSettingsServicesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetWebsiteSettingsServicesRequestBuilder {
    args: Option<SetWebsiteSettingsServicesRequestArgs>,
}

impl SetWebsiteSettingsServicesRequestBuilder {
    pub fn args(mut self, value: SetWebsiteSettingsServicesRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SetWebsiteSettingsServicesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](SetWebsiteSettingsServicesRequestBuilder::args)
    pub fn build(self) -> Result<SetWebsiteSettingsServicesRequest, BuildError> {
        Ok(SetWebsiteSettingsServicesRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

