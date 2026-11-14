pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TiptapMarkFour {
    pub r#type: TiptapMarkFourType,
}

impl TiptapMarkFour {
    pub fn builder() -> TiptapMarkFourBuilder {
        <TiptapMarkFourBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TiptapMarkFourBuilder {
    r#type: Option<TiptapMarkFourType>,
}

impl TiptapMarkFourBuilder {
    pub fn r#type(mut self, value: TiptapMarkFourType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TiptapMarkFour`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](TiptapMarkFourBuilder::r#type)
    pub fn build(self) -> Result<TiptapMarkFour, BuildError> {
        Ok(TiptapMarkFour {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
