use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::utils::normalize;

pub use crate::storage::ids::{AttributeKeyID, EntityID, RelationshipTypeID, SourceID, TagID};

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    Serialize,
    Deserialize,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Debug,
)]
pub struct Date(pub i64);

pub type EntityName = Box<str>;

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, PartialEq, Clone, Debug,
)]
pub struct Tag {
    pub id: TagID,
    pub name: String,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, PartialEq, Clone, Debug,
)]
pub struct Source {
    pub id: SourceID,
    pub name: String,
}

impl Date {
    pub fn now() -> Self {
        Self(chrono::Utc::now().timestamp_millis())
    }

    pub fn from_timestamp(timestamp: i64) -> Self {
        Self(timestamp)
    }

    pub fn timestamp(&self) -> i64 {
        self.0
    }

    pub fn to_chrono(&self) -> chrono::DateTime<chrono::Utc> {
        chrono::DateTime::from_timestamp(self.0, 0).expect("invalid timestamp")
    }
}

pub type Alias = Box<str>;

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq,
)]
pub struct Attribute {
    pub key: AttributeKeyID,
    pub value: Value,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq,
)]
pub enum Value {
    String(Box<str>),
    Integer(i64),
    Float(f64),
    Boolean(bool),
}

impl Value {
    pub fn normalized(&self) -> Box<str> {
        match self {
            Self::String(s) => normalize(s),
            Self::Integer(i) => String::into_boxed_str(i.to_string()),
            Self::Float(f) => String::into_boxed_str(f.to_string()),
            Self::Boolean(b) => String::into_boxed_str(b.to_string()),
        }
    }

    pub fn display_value(&self) -> String {
        match self {
            Self::String(value) => value.to_string(),
            Self::Integer(value) => value.to_string(),
            Self::Float(value) => value.to_string(),
            Self::Boolean(value) => value.to_string(),
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(value) => write!(f, "{value}"),
            Self::Integer(value) => write!(f, "{value}"),
            Self::Float(value) => write!(f, "{value}"),
            Self::Boolean(value) => write!(f, "{value}"),
        }
    }
}

impl ArchivedValue {
    pub fn display_value(&self) -> String {
        match self {
            Self::String(value) => value.to_string(),
            Self::Integer(value) => value.to_string(),
            Self::Float(value) => value.to_string(),
            Self::Boolean(value) => value.to_string(),
        }
    }
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    Serialize,
    Deserialize,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
#[repr(u8)]
pub enum BuiltInRelationship {
    IsA = 0,
    InstanceOf = 1,
    PartOf = 2,
    MadeBy = 3,
    OwnedBy = 4,
    LocatedIn = 5,
    SimilarTo = 6,
    ReplacedBy = 7,
}

impl BuiltInRelationship {
    /// Reserved index range for built-in relationships (0..255).
    pub fn to_id(self) -> RelationshipTypeID {
        RelationshipTypeID::new(self as usize)
    }

    /// Convert from a reserved ID back to a built-in relationship, if valid.
    pub fn from_id(id: RelationshipTypeID) -> Option<Self> {
        match id.index() {
            0 => Some(Self::IsA),
            1 => Some(Self::InstanceOf),
            2 => Some(Self::PartOf),
            3 => Some(Self::MadeBy),
            4 => Some(Self::OwnedBy),
            5 => Some(Self::LocatedIn),
            6 => Some(Self::SimilarTo),
            7 => Some(Self::ReplacedBy),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::IsA => "is_a",
            Self::InstanceOf => "instance_of",
            Self::PartOf => "part_of",
            Self::MadeBy => "made_by",
            Self::OwnedBy => "owned_by",
            Self::LocatedIn => "located_in",
            Self::SimilarTo => "similar_to",
            Self::ReplacedBy => "replaced_by",
        }
    }
}

impl fmt::Display for BuiltInRelationship {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl AsRef<str> for BuiltInRelationship {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    Serialize,
    Deserialize,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum RelationshipType {
    BuiltIn(BuiltInRelationship),
    Custom(RelationshipTypeID),
}

impl RelationshipType {
    pub fn to_id(&self) -> RelationshipTypeID {
        match *self {
            Self::Custom(id) => id,
            Self::BuiltIn(builtin) => builtin.to_id(),
        }
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq, Eq,
)]
pub struct Relationship {
    pub kind: RelationshipType,
    pub target: EntityID,
}

impl std::fmt::Display for RelationshipType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BuiltIn(builtin) => write!(f, "{}", builtin),
            Self::Custom(id) => write!(f, "custom:{}", id),
        }
    }
}
