pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TiptapMarkAttrs {
    #[serde(default)]
    pub attrs: TiptapMarkAttrsAttrs,
    pub r#type: TiptapMarkAttrsType,
}

impl TiptapMarkAttrs {
    pub fn builder() -> TiptapMarkAttrsBuilder {
        <TiptapMarkAttrsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TiptapMarkAttrsBuilder {
    attrs: Option<TiptapMarkAttrsAttrs>,
    r#type: Option<TiptapMarkAttrsType>,
}

impl TiptapMarkAttrsBuilder {
    pub fn attrs(mut self, value: TiptapMarkAttrsAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn r#type(mut self, value: TiptapMarkAttrsType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TiptapMarkAttrs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attrs`](TiptapMarkAttrsBuilder::attrs)
    /// - [`r#type`](TiptapMarkAttrsBuilder::r#type)
    pub fn build(self) -> Result<TiptapMarkAttrs, BuildError> {
        Ok(TiptapMarkAttrs {
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
