pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TiptapDocumentContentItemZeroContentItemTwo {
    #[serde(default)]
    pub attrs: TiptapDocumentContentItemZeroContentItemTwoAttrs,
    pub r#type: TiptapDocumentContentItemZeroContentItemTwoType,
}

impl TiptapDocumentContentItemZeroContentItemTwo {
    pub fn builder() -> TiptapDocumentContentItemZeroContentItemTwoBuilder {
        <TiptapDocumentContentItemZeroContentItemTwoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TiptapDocumentContentItemZeroContentItemTwoBuilder {
    attrs: Option<TiptapDocumentContentItemZeroContentItemTwoAttrs>,
    r#type: Option<TiptapDocumentContentItemZeroContentItemTwoType>,
}

impl TiptapDocumentContentItemZeroContentItemTwoBuilder {
    pub fn attrs(mut self, value: TiptapDocumentContentItemZeroContentItemTwoAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn r#type(mut self, value: TiptapDocumentContentItemZeroContentItemTwoType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TiptapDocumentContentItemZeroContentItemTwo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attrs`](TiptapDocumentContentItemZeroContentItemTwoBuilder::attrs)
    /// - [`r#type`](TiptapDocumentContentItemZeroContentItemTwoBuilder::r#type)
    pub fn build(self) -> Result<TiptapDocumentContentItemZeroContentItemTwo, BuildError> {
        Ok(TiptapDocumentContentItemZeroContentItemTwo {
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
