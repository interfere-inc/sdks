pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TiptapMarkTwo {
    pub r#type: TiptapMarkTwoType,
}

impl TiptapMarkTwo {
    pub fn builder() -> TiptapMarkTwoBuilder {
        <TiptapMarkTwoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TiptapMarkTwoBuilder {
    r#type: Option<TiptapMarkTwoType>,
}

impl TiptapMarkTwoBuilder {
    pub fn r#type(mut self, value: TiptapMarkTwoType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TiptapMarkTwo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](TiptapMarkTwoBuilder::r#type)
    pub fn build(self) -> Result<TiptapMarkTwo, BuildError> {
        Ok(TiptapMarkTwo {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
