use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use serde::{Deserialize, Serialize}; // Bring serde traits into scope

use super::macros::{define_id, define_registry_id};

use crate::utils::normalize;

define_id!(RelationshipTypeID, u16);
define_id!(TagID, u32);
define_id!(SourceID, u32);
define_id!(AttributeKeyID, u32);
define_id!(EntityID, u64);

define_registry_id!(RelationshipTypeID, u16);
define_registry_id!(TagID, u32);
define_registry_id!(SourceID, u32);
define_registry_id!(AttributeKeyID, u32);
define_registry_id!(EntityID, u64);

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
    Entity(EntityID),
}

impl Value {
    pub fn normalized(&self) -> Box<str> {
        match self {
            Self::String(s) => normalize(s),
            Self::Integer(i) => String::into_boxed_str(i.to_string()),
            Self::Float(f) => String::into_boxed_str(f.to_string()),
            Self::Boolean(b) => String::into_boxed_str(b.to_string()),
            Self::Entity(id) => String::into_boxed_str(id.to_string()),
        }
    }

    pub fn display_value(&self) -> String {
        match self {
            Self::String(value) => value.to_string(),
            Self::Integer(value) => value.to_string(),
            Self::Float(value) => value.to_string(),
            Self::Boolean(value) => value.to_string(),
            Self::Entity(value) => value.0.to_string(),
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
            Self::Entity(value) => write!(f, "{value}"),
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
            Self::Entity(value) => value.0.to_string(),
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
)]
pub enum BuiltInRelationship {
    IsA,
    InstanceOf,
    PartOf,
    MadeBy,
    OwnedBy,
    LocatedIn,
    SimilarTo,
    ReplacedBy,
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

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq, Eq,
)]
pub struct Relationship {
    pub kind: RelationshipTypeID,
    pub target: EntityID,
}

impl ArchivedEntityID {
    pub fn to_native(&self) -> EntityID {
        EntityID(self.0.into())
    }
}
