pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TiptapDocumentContentItemZeroContentItemTwoAttrs {
    #[serde(default)]
    pub name: String,
}

impl TiptapDocumentContentItemZeroContentItemTwoAttrs {
    pub fn builder() -> TiptapDocumentContentItemZeroContentItemTwoAttrsBuilder {
        <TiptapDocumentContentItemZeroContentItemTwoAttrsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TiptapDocumentContentItemZeroContentItemTwoAttrsBuilder {
    name: Option<String>,
}

impl TiptapDocumentContentItemZeroContentItemTwoAttrsBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TiptapDocumentContentItemZeroContentItemTwoAttrs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](TiptapDocumentContentItemZeroContentItemTwoAttrsBuilder::name)
    pub fn build(self) -> Result<TiptapDocumentContentItemZeroContentItemTwoAttrs, BuildError> {
        Ok(TiptapDocumentContentItemZeroContentItemTwoAttrs {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
