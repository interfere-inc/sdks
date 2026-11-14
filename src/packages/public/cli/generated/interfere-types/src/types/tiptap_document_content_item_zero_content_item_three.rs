pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TiptapDocumentContentItemZeroContentItemThree {
    pub r#type: TiptapDocumentContentItemZeroContentItemThreeType,
}

impl TiptapDocumentContentItemZeroContentItemThree {
    pub fn builder() -> TiptapDocumentContentItemZeroContentItemThreeBuilder {
        <TiptapDocumentContentItemZeroContentItemThreeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TiptapDocumentContentItemZeroContentItemThreeBuilder {
    r#type: Option<TiptapDocumentContentItemZeroContentItemThreeType>,
}

impl TiptapDocumentContentItemZeroContentItemThreeBuilder {
    pub fn r#type(mut self, value: TiptapDocumentContentItemZeroContentItemThreeType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TiptapDocumentContentItemZeroContentItemThree`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](TiptapDocumentContentItemZeroContentItemThreeBuilder::r#type)
    pub fn build(self) -> Result<TiptapDocumentContentItemZeroContentItemThree, BuildError> {
        Ok(TiptapDocumentContentItemZeroContentItemThree {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
