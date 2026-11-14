pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TiptapMarkAttrsAttrs {
    #[serde(default)]
    pub href: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<TiptapMarkAttrsAttrsTarget>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rel: Option<String>,
}

impl TiptapMarkAttrsAttrs {
    pub fn builder() -> TiptapMarkAttrsAttrsBuilder {
        <TiptapMarkAttrsAttrsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TiptapMarkAttrsAttrsBuilder {
    href: Option<String>,
    target: Option<TiptapMarkAttrsAttrsTarget>,
    rel: Option<String>,
}

impl TiptapMarkAttrsAttrsBuilder {
    pub fn href(mut self, value: impl Into<String>) -> Self {
        self.href = Some(value.into());
        self
    }

    pub fn target(mut self, value: TiptapMarkAttrsAttrsTarget) -> Self {
        self.target = Some(value);
        self
    }

    pub fn rel(mut self, value: impl Into<String>) -> Self {
        self.rel = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TiptapMarkAttrsAttrs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`href`](TiptapMarkAttrsAttrsBuilder::href)
    pub fn build(self) -> Result<TiptapMarkAttrsAttrs, BuildError> {
        Ok(TiptapMarkAttrsAttrs {
            href: self.href.ok_or_else(|| BuildError::missing_field("href"))?,
            target: self.target,
            rel: self.rel,
        })
    }
}
