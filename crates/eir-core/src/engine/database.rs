use rkyv::{Archive, Deserialize, Serialize};

use crate::entity::EntityDocument;

#[derive(Archive, Serialize, Deserialize, Debug)]
pub struct Database {
    pub entities: Vec<EntityDocument>,

    pub tags: Vec<Box<str>>,
    pub properties: Vec<Box<str>>,
    pub sources: Vec<Box<str>>,
}
