use crate::{
    index::Resolver,
    query::Filter,
    search::{candidate::CandidateSet, signal::Signal},
};

pub fn execute(resolver: &Resolver, candidates: &mut CandidateSet, filter: &Filter) {
    let Filter::Relationship { kind, target } = filter else {
        return;
    };

    let Some(kind_id) = resolver.relationship_type_id(kind) else {
        return;
    };

    let Some(target_id) = resolver.resolve(target).first() else {
        return;
    };

    for entity_id in resolver.relationship_lookup(kind_id, *target_id) {
        candidates.add_signal(entity_id, Signal::Relationship);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        entity::prelude::{
            EntityDocument,
            types::{EntityID, Relationship, RelationshipType},
        },
        index::Resolver,
        query::Filter,
        search::{CandidateSet, signal::Signal},
        test_utils::test_entity,
    };

    #[test]
    fn relationship_adds_candidate() {
        let mut resolver = Resolver::default();

        let entity_id_1 = EntityID::new(1);
        let entity_id_2 = EntityID::new(10);

        let manufacturer_type = resolver.register_relationship_type("manufacturer");

        resolver.add(test_entity(entity_id_2, "Nestle"));

        let product = EntityDocument {
            id: entity_id_1,
            aliases: vec!["Chocolate Bar".into()],
            relationships: vec![Relationship {
                kind: RelationshipType::Custom(manufacturer_type),
                target: entity_id_2,
            }],
            attributes: Vec::new(),
            sources: Vec::new(),
            tags: Vec::new(),
        };

        resolver.add(product);

        let filter = Filter::Relationship {
            kind: "manufacturer".into(),
            target: "nestle".into(),
        };

        let mut candidates = CandidateSet::default();

        execute(&resolver, &mut candidates, &filter);

        let candidate = candidates.get(entity_id_1).unwrap();

        assert!(candidate.signals.contains(&Signal::Relationship));
    }

    #[test]
    fn relationship_does_not_match_unknown_target() {
        let mut resolver = Resolver::default();

        let entity_id = EntityID::new(1);
        let target_id = EntityID::new(10);

        let kind = resolver.register_relationship_type("manufacturer");

        resolver.add(test_entity(target_id, "Nestle"));

        resolver.add(EntityDocument {
            id: entity_id,
            aliases: vec!["Chocolate Bar".into()],
            relationships: vec![Relationship {
                kind: RelationshipType::Custom(kind),
                target: target_id,
            }],
            attributes: Vec::new(),
            sources: Vec::new(),
            tags: Vec::new(),
        });

        let filter = Filter::Relationship {
            kind: "manufacturer".into(),
            target: "Unknown".into(),
        };

        let mut candidates = CandidateSet::default();

        execute(&resolver, &mut candidates, &filter);

        assert!(candidates.get(entity_id).is_none());
    }

    #[test]
    fn relationship_does_not_match_wrong_kind() {
        let mut resolver = Resolver::default();

        let entity_id = EntityID::new(1);
        let target_id = EntityID::new(10);

        let manufacturer = resolver.register_relationship_type("manufacturer");

        resolver.register_relationship_type("owner");

        resolver.add(test_entity(target_id, "Nestle"));

        resolver.add(EntityDocument {
            id: entity_id,
            aliases: vec!["Chocolate Bar".into()],
            relationships: vec![Relationship {
                kind: RelationshipType::Custom(manufacturer),
                target: target_id,
            }],
            attributes: Vec::new(),
            sources: Vec::new(),
            tags: Vec::new(),
        });

        let filter = Filter::Relationship {
            kind: "owner".into(),
            target: "nestle".into(),
        };

        let mut candidates = CandidateSet::default();

        execute(&resolver, &mut candidates, &filter);

        assert!(candidates.get(entity_id).is_none());
    }
}
