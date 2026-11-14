pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum TiptapDocumentContentItem {
        TiptapDocumentContentItemZero(TiptapDocumentContentItemZero),

        TiptapDocumentContentItemOne(TiptapDocumentContentItemOne),

        TiptapDocumentContentItemType(TiptapDocumentContentItemType),
}

impl TiptapDocumentContentItem {
    pub fn is_tiptap_document_content_item_zero(&self) -> bool {
        matches!(self, Self::TiptapDocumentContentItemZero(_))
    }

    pub fn is_tiptap_document_content_item_one(&self) -> bool {
        matches!(self, Self::TiptapDocumentContentItemOne(_))
    }

    pub fn is_tiptap_document_content_item_type(&self) -> bool {
        matches!(self, Self::TiptapDocumentContentItemType(_))
    }


    pub fn as_tiptap_document_content_item_zero(&self) -> Option<&TiptapDocumentContentItemZero> {
        match self {
                    Self::TiptapDocumentContentItemZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_tiptap_document_content_item_zero(self) -> Option<TiptapDocumentContentItemZero> {
        match self {
                    Self::TiptapDocumentContentItemZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_tiptap_document_content_item_one(&self) -> Option<&TiptapDocumentContentItemOne> {
        match self {
                    Self::TiptapDocumentContentItemOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_tiptap_document_content_item_one(self) -> Option<TiptapDocumentContentItemOne> {
        match self {
                    Self::TiptapDocumentContentItemOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_tiptap_document_content_item_type(&self) -> Option<&TiptapDocumentContentItemType> {
        match self {
                    Self::TiptapDocumentContentItemType(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_tiptap_document_content_item_type(self) -> Option<TiptapDocumentContentItemType> {
        match self {
                    Self::TiptapDocumentContentItemType(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for TiptapDocumentContentItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TiptapDocumentContentItemZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TiptapDocumentContentItemOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TiptapDocumentContentItemType(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
