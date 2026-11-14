pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateTiersRequestPatch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shade: Option<UpdateTiersRequestPatchShade>,
}

impl UpdateTiersRequestPatch {
    pub fn builder() -> UpdateTiersRequestPatchBuilder {
        <UpdateTiersRequestPatchBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateTiersRequestPatchBuilder {
    value: Option<String>,
    shade: Option<UpdateTiersRequestPatchShade>,
}

impl UpdateTiersRequestPatchBuilder {
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn shade(mut self, value: UpdateTiersRequestPatchShade) -> Self {
        self.shade = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateTiersRequestPatch`].
    pub fn build(self) -> Result<UpdateTiersRequestPatch, BuildError> {
        Ok(UpdateTiersRequestPatch {
            value: self.value,
            shade: self.shade,
        })
    }
}
