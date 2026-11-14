pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TiptapMarkOne {
    pub r#type: TiptapMarkOneType,
}

impl TiptapMarkOne {
    pub fn builder() -> TiptapMarkOneBuilder {
        <TiptapMarkOneBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TiptapMarkOneBuilder {
    r#type: Option<TiptapMarkOneType>,
}

impl TiptapMarkOneBuilder {
    pub fn r#type(mut self, value: TiptapMarkOneType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TiptapMarkOne`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](TiptapMarkOneBuilder::r#type)
    pub fn build(self) -> Result<TiptapMarkOne, BuildError> {
        Ok(TiptapMarkOne {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
