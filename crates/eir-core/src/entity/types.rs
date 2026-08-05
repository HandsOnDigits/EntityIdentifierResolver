use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use serde::{Deserialize, Serialize}; // Bring serde traits into scope

pub type EntityID = u64;

pub type TagID = u32;

use crate::utils::normalize;

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, PartialEq, Clone, Debug,
)]
pub struct Tag {
    pub id: TagID,
    pub name: String,
}

pub type SourceID = u32;

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, PartialEq, Clone, Debug,
)]
pub struct Source {
    pub id: SourceID,
    pub name: String,
}

pub type EntityName = Box<str>;

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

pub type AttributeKeyID = u32;

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq,
)]
pub struct Attribute {
    pub key: AttributeKeyID,
    pub value: Value,
}

impl ArchivedValue {
    pub fn display_value(&self) -> String {
        match self {
            Self::String(value) => value.to_string(),
            Self::Integer(value) => value.to_string(),
            Self::Float(value) => value.to_string(),
            Self::Boolean(value) => value.to_string(),
            Self::Entity(value) => value.to_string(),
        }
    }
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
}

#[derive(Debug)]
pub enum EntityError {
    Io(std::io::Error),
    Serialize(rkyv::rancor::Error),
}

impl From<std::io::Error> for EntityError {
    fn from(error: std::io::Error) -> Self {
        EntityError::Io(error)
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
    IsA,
    InstanceOf,
    PartOf,
    MadeBy,
    OwnedBy,
    LocatedIn,
    SimilarTo,
    ReplacedBy,
}

impl ArchivedRelationshipType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::IsA => "IsA",
            Self::InstanceOf => "InstanceOf",
            Self::PartOf => "PartOf",
            Self::MadeBy => "MadeBy",
            Self::OwnedBy => "OwnedBy",
            Self::LocatedIn => "LocatedIn",
            Self::SimilarTo => "SimilarTo",
            Self::ReplacedBy => "ReplacedBy",
        }
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq, Eq,
)]
pub struct Relationship {
    pub target: EntityID,
    pub kind: RelationshipType,
}

pub struct EntityIndexEntry {
    pub id: u64,
    pub offset: u64,
    pub size: u32,
}
