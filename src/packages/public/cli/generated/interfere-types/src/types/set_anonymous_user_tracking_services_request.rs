pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SetAnonymousUserTrackingServicesRequest {
    #[serde(default)]
    pub args: SetAnonymousUserTrackingServicesRequestArgs,
}

impl SetAnonymousUserTrackingServicesRequest {
    pub fn builder() -> SetAnonymousUserTrackingServicesRequestBuilder {
        <SetAnonymousUserTrackingServicesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetAnonymousUserTrackingServicesRequestBuilder {
    args: Option<SetAnonymousUserTrackingServicesRequestArgs>,
}

impl SetAnonymousUserTrackingServicesRequestBuilder {
    pub fn args(mut self, value: SetAnonymousUserTrackingServicesRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SetAnonymousUserTrackingServicesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](SetAnonymousUserTrackingServicesRequestBuilder::args)
    pub fn build(self) -> Result<SetAnonymousUserTrackingServicesRequest, BuildError> {
        Ok(SetAnonymousUserTrackingServicesRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

