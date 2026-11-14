pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum TiptapDocumentContentItemZeroContentItem {
        TiptapDocumentContentItemZeroContentItemMarks(TiptapDocumentContentItemZeroContentItemMarks),

        TiptapDocumentContentItemZeroContentItemOne(TiptapDocumentContentItemZeroContentItemOne),

        TiptapDocumentContentItemZeroContentItemTwo(TiptapDocumentContentItemZeroContentItemTwo),

        TiptapDocumentContentItemZeroContentItemThree(TiptapDocumentContentItemZeroContentItemThree),

        TiptapDocumentContentItemZeroContentItemFour(TiptapDocumentContentItemZeroContentItemFour),

        TiptapDocumentContentItemZeroContentItemFive(TiptapDocumentContentItemZeroContentItemFive),

        TiptapDocumentContentItemZeroContentItemSix(TiptapDocumentContentItemZeroContentItemSix),

        TiptapDocumentContentItemZeroContentItemSeven(TiptapDocumentContentItemZeroContentItemSeven),

        TiptapDocumentContentItemZeroContentItemEight(TiptapDocumentContentItemZeroContentItemEight),

        TiptapDocumentContentItemZeroContentItemNine(TiptapDocumentContentItemZeroContentItemNine),
}

impl TiptapDocumentContentItemZeroContentItem {
    pub fn is_tiptap_document_content_item_zero_content_item_marks(&self) -> bool {
        matches!(self, Self::TiptapDocumentContentItemZeroContentItemMarks(_))
    }

    pub fn is_tiptap_document_content_item_zero_content_item_one(&self) -> bool {
        matches!(self, Self::TiptapDocumentContentItemZeroContentItemOne(_))
    }

    pub fn is_tiptap_document_content_item_zero_content_item_two(&self) -> bool {
        matches!(self, Self::TiptapDocumentContentItemZeroContentItemTwo(_))
    }

    pub fn is_tiptap_document_content_item_zero_content_item_three(&self) -> bool {
        matches!(self, Self::TiptapDocumentContentItemZeroContentItemThree(_))
    }

    pub fn is_tiptap_document_content_item_zero_content_item_four(&self) -> bool {
        matches!(self, Self::TiptapDocumentContentItemZeroContentItemFour(_))
    }

    pub fn is_tiptap_document_content_item_zero_content_item_five(&self) -> bool {
        matches!(self, Self::TiptapDocumentContentItemZeroContentItemFive(_))
    }

    pub fn is_tiptap_document_content_item_zero_content_item_six(&self) -> bool {
        matches!(self, Self::TiptapDocumentContentItemZeroContentItemSix(_))
    }

    pub fn is_tiptap_document_content_item_zero_content_item_seven(&self) -> bool {
        matches!(self, Self::TiptapDocumentContentItemZeroContentItemSeven(_))
    }

    pub fn is_tiptap_document_content_item_zero_content_item_eight(&self) -> bool {
        matches!(self, Self::TiptapDocumentContentItemZeroContentItemEight(_))
    }

    pub fn is_tiptap_document_content_item_zero_content_item_nine(&self) -> bool {
        matches!(self, Self::TiptapDocumentContentItemZeroContentItemNine(_))
    }


    pub fn as_tiptap_document_content_item_zero_content_item_marks(&self) -> Option<&TiptapDocumentContentItemZeroContentItemMarks> {
        match self {
                    Self::TiptapDocumentContentItemZeroContentItemMarks(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_tiptap_document_content_item_zero_content_item_marks(self) -> Option<TiptapDocumentContentItemZeroContentItemMarks> {
        match self {
                    Self::TiptapDocumentContentItemZeroContentItemMarks(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_tiptap_document_content_item_zero_content_item_one(&self) -> Option<&TiptapDocumentContentItemZeroContentItemOne> {
        match self {
                    Self::TiptapDocumentContentItemZeroContentItemOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_tiptap_document_content_item_zero_content_item_one(self) -> Option<TiptapDocumentContentItemZeroContentItemOne> {
        match self {
                    Self::TiptapDocumentContentItemZeroContentItemOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_tiptap_document_content_item_zero_content_item_two(&self) -> Option<&TiptapDocumentContentItemZeroContentItemTwo> {
        match self {
                    Self::TiptapDocumentContentItemZeroContentItemTwo(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_tiptap_document_content_item_zero_content_item_two(self) -> Option<TiptapDocumentContentItemZeroContentItemTwo> {
        match self {
                    Self::TiptapDocumentContentItemZeroContentItemTwo(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_tiptap_document_content_item_zero_content_item_three(&self) -> Option<&TiptapDocumentContentItemZeroContentItemThree> {
        match self {
                    Self::TiptapDocumentContentItemZeroContentItemThree(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_tiptap_document_content_item_zero_content_item_three(self) -> Option<TiptapDocumentContentItemZeroContentItemThree> {
        match self {
                    Self::TiptapDocumentContentItemZeroContentItemThree(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_tiptap_document_content_item_zero_content_item_four(&self) -> Option<&TiptapDocumentContentItemZeroContentItemFour> {
        match self {
                    Self::TiptapDocumentContentItemZeroContentItemFour(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_tiptap_document_content_item_zero_content_item_four(self) -> Option<TiptapDocumentContentItemZeroContentItemFour> {
        match self {
                    Self::TiptapDocumentContentItemZeroContentItemFour(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_tiptap_document_content_item_zero_content_item_five(&self) -> Option<&TiptapDocumentContentItemZeroContentItemFive> {
        match self {
                    Self::TiptapDocumentContentItemZeroContentItemFive(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_tiptap_document_content_item_zero_content_item_five(self) -> Option<TiptapDocumentContentItemZeroContentItemFive> {
        match self {
                    Self::TiptapDocumentContentItemZeroContentItemFive(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_tiptap_document_content_item_zero_content_item_six(&self) -> Option<&TiptapDocumentContentItemZeroContentItemSix> {
        match self {
                    Self::TiptapDocumentContentItemZeroContentItemSix(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_tiptap_document_content_item_zero_content_item_six(self) -> Option<TiptapDocumentContentItemZeroContentItemSix> {
        match self {
                    Self::TiptapDocumentContentItemZeroContentItemSix(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_tiptap_document_content_item_zero_content_item_seven(&self) -> Option<&TiptapDocumentContentItemZeroContentItemSeven> {
        match self {
                    Self::TiptapDocumentContentItemZeroContentItemSeven(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_tiptap_document_content_item_zero_content_item_seven(self) -> Option<TiptapDocumentContentItemZeroContentItemSeven> {
        match self {
                    Self::TiptapDocumentContentItemZeroContentItemSeven(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_tiptap_document_content_item_zero_content_item_eight(&self) -> Option<&TiptapDocumentContentItemZeroContentItemEight> {
        match self {
                    Self::TiptapDocumentContentItemZeroContentItemEight(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_tiptap_document_content_item_zero_content_item_eight(self) -> Option<TiptapDocumentContentItemZeroContentItemEight> {
        match self {
                    Self::TiptapDocumentContentItemZeroContentItemEight(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_tiptap_document_content_item_zero_content_item_nine(&self) -> Option<&TiptapDocumentContentItemZeroContentItemNine> {
        match self {
                    Self::TiptapDocumentContentItemZeroContentItemNine(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_tiptap_document_content_item_zero_content_item_nine(self) -> Option<TiptapDocumentContentItemZeroContentItemNine> {
        match self {
                    Self::TiptapDocumentContentItemZeroContentItemNine(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for TiptapDocumentContentItemZeroContentItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TiptapDocumentContentItemZeroContentItemMarks(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TiptapDocumentContentItemZeroContentItemOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TiptapDocumentContentItemZeroContentItemTwo(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TiptapDocumentContentItemZeroContentItemThree(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TiptapDocumentContentItemZeroContentItemFour(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TiptapDocumentContentItemZeroContentItemFive(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TiptapDocumentContentItemZeroContentItemSix(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TiptapDocumentContentItemZeroContentItemSeven(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TiptapDocumentContentItemZeroContentItemEight(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TiptapDocumentContentItemZeroContentItemNine(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
