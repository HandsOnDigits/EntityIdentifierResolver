use rkyv::{Archive, Deserialize, Serialize};

pub type EntityID = u64;

pub type TagID = u32;

#[derive(Archive, Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Tag {
    pub id: TagID,
    pub name: String,
}

pub type SourceID = u32;

#[derive(Archive, Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Source {
    pub id: SourceID,
    pub name: String,
}

pub type EntityType = [u8; 4];

pub type EntityName = Box<str>;

#[derive(Archive, Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
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

pub type PropertyID = u32;

#[derive(Archive, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Property {
    pub key: PropertyID,
    pub value: Value,
}

#[derive(Archive, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Entity(EntityID),
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

#[derive(Archive, Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Archive, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Relationship {
    pub target: EntityID,
    pub kind: RelationshipType,
}

pub struct EntityIndexEntry {
    pub id: u64,
    pub offset: u64,
    pub size: u32,
}
