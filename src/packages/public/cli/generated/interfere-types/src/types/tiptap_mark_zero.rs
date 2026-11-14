pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TiptapMarkZero {
    pub r#type: TiptapMarkZeroType,
}

impl TiptapMarkZero {
    pub fn builder() -> TiptapMarkZeroBuilder {
        <TiptapMarkZeroBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TiptapMarkZeroBuilder {
    r#type: Option<TiptapMarkZeroType>,
}

impl TiptapMarkZeroBuilder {
    pub fn r#type(mut self, value: TiptapMarkZeroType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TiptapMarkZero`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](TiptapMarkZeroBuilder::r#type)
    pub fn build(self) -> Result<TiptapMarkZero, BuildError> {
        Ok(TiptapMarkZero {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
