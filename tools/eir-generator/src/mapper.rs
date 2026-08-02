use eir_core::entity::types::*;

pub fn map_relationship(value: &str) -> RelationshipType {
    match value {
        "made_by" => RelationshipType::MadeBy,

        "is_a" => RelationshipType::IsA,

        "located_in" => RelationshipType::LocatedIn,

        "part_of" => RelationshipType::PartOf,

        _ => RelationshipType::SimilarTo,
    }
}

pub fn map_entity_type(value: &str) -> EntityType {
    match value {
        "food" => *b"FOOD",
        "company" => *b"COMP",
        "category" => *b"CATG",
        "country" => *b"CTRY",
        _ => *b"UNKN",
    }
}
