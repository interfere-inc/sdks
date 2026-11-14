pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TiptapMarkType {
    pub r#type: TiptapMarkTypeType,
}

impl TiptapMarkType {
    pub fn builder() -> TiptapMarkTypeBuilder {
        <TiptapMarkTypeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TiptapMarkTypeBuilder {
    r#type: Option<TiptapMarkTypeType>,
}

impl TiptapMarkTypeBuilder {
    pub fn r#type(mut self, value: TiptapMarkTypeType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TiptapMarkType`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](TiptapMarkTypeBuilder::r#type)
    pub fn build(self) -> Result<TiptapMarkType, BuildError> {
        Ok(TiptapMarkType {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
