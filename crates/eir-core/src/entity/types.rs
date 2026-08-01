#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityID(pub u64);

pub type EntityType = [u8; 4];

pub type Date = chrono::DateTime<chrono::Utc>;

pub type Alias = Box<str>;

pub type Tag = u32;

pub type PropertyID = u32;

pub enum Value {
    String(Box<str>),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Entity(EntityID),
}

pub struct Property {
    pub key: PropertyID,
    pub value: Value,
}

pub enum RelationshipType {
    Parent,
    Child,
    Manufacturer,
    CreatedBy,
    SimilarTo,
}

pub struct Relationship {
    pub target: EntityID,
    pub kind: RelationshipType,
}
