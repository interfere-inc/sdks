pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SetAnonymousUserTrackingSurfacesRequest {
    #[serde(default)]
    pub args: SetAnonymousUserTrackingSurfacesRequestArgs,
}

impl SetAnonymousUserTrackingSurfacesRequest {
    pub fn builder() -> SetAnonymousUserTrackingSurfacesRequestBuilder {
        <SetAnonymousUserTrackingSurfacesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetAnonymousUserTrackingSurfacesRequestBuilder {
    args: Option<SetAnonymousUserTrackingSurfacesRequestArgs>,
}

impl SetAnonymousUserTrackingSurfacesRequestBuilder {
    pub fn args(mut self, value: SetAnonymousUserTrackingSurfacesRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SetAnonymousUserTrackingSurfacesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](SetAnonymousUserTrackingSurfacesRequestBuilder::args)
    pub fn build(self) -> Result<SetAnonymousUserTrackingSurfacesRequest, BuildError> {
        Ok(SetAnonymousUserTrackingSurfacesRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

