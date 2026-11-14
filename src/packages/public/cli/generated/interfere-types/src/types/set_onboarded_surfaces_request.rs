pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SetOnboardedSurfacesRequest {
    #[serde(default)]
    pub args: SetOnboardedSurfacesRequestArgs,
}

impl SetOnboardedSurfacesRequest {
    pub fn builder() -> SetOnboardedSurfacesRequestBuilder {
        <SetOnboardedSurfacesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetOnboardedSurfacesRequestBuilder {
    args: Option<SetOnboardedSurfacesRequestArgs>,
}

impl SetOnboardedSurfacesRequestBuilder {
    pub fn args(mut self, value: SetOnboardedSurfacesRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SetOnboardedSurfacesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](SetOnboardedSurfacesRequestBuilder::args)
    pub fn build(self) -> Result<SetOnboardedSurfacesRequest, BuildError> {
        Ok(SetOnboardedSurfacesRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

