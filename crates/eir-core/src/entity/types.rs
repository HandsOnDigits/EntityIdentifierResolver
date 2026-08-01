use rkyv::{Archive, Deserialize, Serialize};

#[derive(Archive, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EntityID(pub u64);

pub type EntityType = [u8; 4];

pub type EntityName = String;

pub type Date = chrono::DateTime<chrono::Utc>;

pub type Alias = Box<str>;

pub type Tag = u32;

pub type PropertyID = u32;

#[derive(Archive, Serialize, Deserialize)]
pub enum Value {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Entity(EntityID),
}

#[derive(Archive, Serialize, Deserialize)]
pub struct Property {
    pub key: PropertyID,
    pub value: Value,
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

#[derive(Archive, Serialize, Deserialize)]
pub enum RelationshipType {
    Parent,
    Child,
    Manufacturer,
    CreatedBy,
    SimilarTo,
}

#[derive(Archive, Serialize, Deserialize)]
pub struct Relationship {
    pub target: EntityID,
    pub kind: RelationshipType,
}

pub struct EntityIndexEntry {
    pub id: u64,
    pub offset: u64,
    pub size: u32,
}
