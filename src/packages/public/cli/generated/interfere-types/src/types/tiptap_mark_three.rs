pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TiptapMarkThree {
    pub r#type: TiptapMarkThreeType,
}

impl TiptapMarkThree {
    pub fn builder() -> TiptapMarkThreeBuilder {
        <TiptapMarkThreeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TiptapMarkThreeBuilder {
    r#type: Option<TiptapMarkThreeType>,
}

impl TiptapMarkThreeBuilder {
    pub fn r#type(mut self, value: TiptapMarkThreeType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TiptapMarkThree`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](TiptapMarkThreeBuilder::r#type)
    pub fn build(self) -> Result<TiptapMarkThree, BuildError> {
        Ok(TiptapMarkThree {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
