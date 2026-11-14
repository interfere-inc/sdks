pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TiptapDocumentContentItemZeroContentItemFive {
    pub attrs: TiptapDocumentContentItemZeroContentItemFiveAttrs,
    pub r#type: TiptapDocumentContentItemZeroContentItemFiveType,
}

impl TiptapDocumentContentItemZeroContentItemFive {
    pub fn builder() -> TiptapDocumentContentItemZeroContentItemFiveBuilder {
        <TiptapDocumentContentItemZeroContentItemFiveBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TiptapDocumentContentItemZeroContentItemFiveBuilder {
    attrs: Option<TiptapDocumentContentItemZeroContentItemFiveAttrs>,
    r#type: Option<TiptapDocumentContentItemZeroContentItemFiveType>,
}

impl TiptapDocumentContentItemZeroContentItemFiveBuilder {
    pub fn attrs(mut self, value: TiptapDocumentContentItemZeroContentItemFiveAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn r#type(mut self, value: TiptapDocumentContentItemZeroContentItemFiveType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TiptapDocumentContentItemZeroContentItemFive`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attrs`](TiptapDocumentContentItemZeroContentItemFiveBuilder::attrs)
    /// - [`r#type`](TiptapDocumentContentItemZeroContentItemFiveBuilder::r#type)
    pub fn build(self) -> Result<TiptapDocumentContentItemZeroContentItemFive, BuildError> {
        Ok(TiptapDocumentContentItemZeroContentItemFive {
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
