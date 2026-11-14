pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum TiptapMark {
        TiptapMarkZero(TiptapMarkZero),

        TiptapMarkOne(TiptapMarkOne),

        TiptapMarkTwo(TiptapMarkTwo),

        TiptapMarkThree(TiptapMarkThree),

        TiptapMarkFour(TiptapMarkFour),

        TiptapMarkAttrs(TiptapMarkAttrs),

        TiptapMarkType(TiptapMarkType),
}

impl TiptapMark {
    pub fn is_tiptap_mark_zero(&self) -> bool {
        matches!(self, Self::TiptapMarkZero(_))
    }

    pub fn is_tiptap_mark_one(&self) -> bool {
        matches!(self, Self::TiptapMarkOne(_))
    }

    pub fn is_tiptap_mark_two(&self) -> bool {
        matches!(self, Self::TiptapMarkTwo(_))
    }

    pub fn is_tiptap_mark_three(&self) -> bool {
        matches!(self, Self::TiptapMarkThree(_))
    }

    pub fn is_tiptap_mark_four(&self) -> bool {
        matches!(self, Self::TiptapMarkFour(_))
    }

    pub fn is_tiptap_mark_attrs(&self) -> bool {
        matches!(self, Self::TiptapMarkAttrs(_))
    }

    pub fn is_tiptap_mark_type(&self) -> bool {
        matches!(self, Self::TiptapMarkType(_))
    }


    pub fn as_tiptap_mark_zero(&self) -> Option<&TiptapMarkZero> {
        match self {
                    Self::TiptapMarkZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_tiptap_mark_zero(self) -> Option<TiptapMarkZero> {
        match self {
                    Self::TiptapMarkZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_tiptap_mark_one(&self) -> Option<&TiptapMarkOne> {
        match self {
                    Self::TiptapMarkOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_tiptap_mark_one(self) -> Option<TiptapMarkOne> {
        match self {
                    Self::TiptapMarkOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_tiptap_mark_two(&self) -> Option<&TiptapMarkTwo> {
        match self {
                    Self::TiptapMarkTwo(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_tiptap_mark_two(self) -> Option<TiptapMarkTwo> {
        match self {
                    Self::TiptapMarkTwo(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_tiptap_mark_three(&self) -> Option<&TiptapMarkThree> {
        match self {
                    Self::TiptapMarkThree(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_tiptap_mark_three(self) -> Option<TiptapMarkThree> {
        match self {
                    Self::TiptapMarkThree(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_tiptap_mark_four(&self) -> Option<&TiptapMarkFour> {
        match self {
                    Self::TiptapMarkFour(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_tiptap_mark_four(self) -> Option<TiptapMarkFour> {
        match self {
                    Self::TiptapMarkFour(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_tiptap_mark_attrs(&self) -> Option<&TiptapMarkAttrs> {
        match self {
                    Self::TiptapMarkAttrs(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_tiptap_mark_attrs(self) -> Option<TiptapMarkAttrs> {
        match self {
                    Self::TiptapMarkAttrs(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_tiptap_mark_type(&self) -> Option<&TiptapMarkType> {
        match self {
                    Self::TiptapMarkType(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_tiptap_mark_type(self) -> Option<TiptapMarkType> {
        match self {
                    Self::TiptapMarkType(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for TiptapMark {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TiptapMarkZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TiptapMarkOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TiptapMarkTwo(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TiptapMarkThree(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TiptapMarkFour(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TiptapMarkAttrs(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TiptapMarkType(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
