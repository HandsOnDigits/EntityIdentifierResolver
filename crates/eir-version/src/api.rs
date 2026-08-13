use std::path::Path;

use eir_core::engine::Engine;

use super::{
    error::{Error, Result},
    merge::merge_databases,
    validate::validate_merge_paths,
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct MergeReport {
    pub entities_added: usize,
    pub entities_skipped: usize,
}

fn create_output_engine(path: &Path) -> Result<Engine> {
    if path.exists() {
        return Err(Error::OutputExists(path.display().to_string()));
    }

    let root = path.parent().ok_or(Error::InvalidOutputPath)?;

    let name = path
        .file_stem()
        .and_then(|value| value.to_str())
        .ok_or(Error::InvalidOutputPath)?;

    let parent = root.parent().ok_or(Error::InvalidOutputPath)?;

    Engine::create(parent, name).map_err(Into::into)
}

pub fn merge(
    left: impl AsRef<Path>,
    right: impl AsRef<Path>,
    output: impl AsRef<Path>,
) -> Result<MergeReport> {
    let left = left.as_ref();
    let right = right.as_ref();
    let output = output.as_ref();

    validate_merge_paths(left, right, output)?;

    let left_engine = Engine::open(left)?;
    let right_engine = Engine::open(right)?;

    let mut output_engine = create_output_engine(output)?;

    merge_databases(&left_engine, &right_engine, &mut output_engine)
}

#[cfg(test)]
mod test {
    use super::*;

    use tempfile::tempdir;

    use eir_core::entity::{
        input::EntityInput,
        prelude::types::{EntityID, RelationshipType},
    };

    #[test]
    fn merge_combines_two_databases() -> Result<()> {
        let temp = tempdir()?;

        let left = create_database(temp.path(), "left", 1000, "Left Entity")?;
        let right = create_database(temp.path(), "right", 2000, "Right Entity")?;
        let output = temp.path().join("merged").join("merged.eir");

        let report = merge(&left, &right, &output)?;

        assert_eq!(report.entities_added, 2);
        assert_eq!(report.entities_skipped, 0);

        let merged = Engine::open(&output)?;

        assert!(merged.entity(EntityID::new(1000)).is_some());
        assert!(merged.entity(EntityID::new(2000)).is_some());

        Ok(())
    }

    #[test]
    fn merge_skips_duplicate_entity_ids() -> Result<()> {
        let temp = tempdir()?;

        let left = create_database(temp.path(), "left", 1000, "Left Entity")?;
        let right = create_database(temp.path(), "right", 1000, "Right Entity")?;
        let output = temp.path().join("merged").join("merged.eir");

        let report = merge(&left, &right, &output)?;

        assert_eq!(report.entities_added, 1);
        assert_eq!(report.entities_skipped, 1);

        let merged = Engine::open(&output)?;

        assert_eq!(
            merged.entity(EntityID::new(1000)).unwrap().aliases,
            vec!["Left Entity".into()]
        );

        Ok(())
    }

    fn create_database(
        parent: &Path,
        name: &str,
        id: usize,
        alias: &str,
    ) -> Result<std::path::PathBuf> {
        let mut engine = Engine::create(parent, name)?;

        engine.insert(EntityInput {
            id: EntityID::new(id),
            aliases: vec![alias.into()],
            tags: vec![],
            sources: vec![],
            attributes: vec![],
            relationships: vec![],
        })?;

        assert_eq!(engine.database().entities.len(), 1);

        engine.flush()?;

        let path = parent.join(name).join(format!("{name}.eir"));

        drop(engine);

        let reopened = Engine::open(&path)?;

        assert_eq!(reopened.database().entities.len(), 1);
        assert!(reopened.entity(EntityID::new(id)).is_some());

        Ok(path)
    }

    #[test]
    fn merge_remaps_tags_and_sources() -> Result<()> {
        let temp = tempdir()?;

        let left = create_database_with_metadata(
            temp.path(),
            "left",
            1000,
            "Left Entity",
            vec!["Food"],
            vec!["Open Food Facts"],
        )?;

        let right = create_database_with_metadata(
            temp.path(),
            "right",
            2000,
            "Right Entity",
            vec!["Food", "Manufacturer"],
            vec!["Manufacturer Registry"],
        )?;

        let output = temp.path().join("merged").join("merged.eir");

        let report = merge(&left, &right, &output)?;

        assert_eq!(report.entities_added, 2);
        assert_eq!(report.entities_skipped, 0);

        let merged = Engine::open(&output)?;

        let left_entity = merged.entity(EntityID::new(1000)).unwrap();
        let right_entity = merged.entity(EntityID::new(2000)).unwrap();

        let left_tags: Vec<_> = left_entity
            .tags
            .iter()
            .filter_map(|id| merged.database().tags.get(*id))
            .collect();

        let right_tags: Vec<_> = right_entity
            .tags
            .iter()
            .filter_map(|id| merged.database().tags.get(*id))
            .collect();

        assert_eq!(left_tags, vec!["food"]);
        assert_eq!(right_tags, vec!["food", "manufacturer"]);

        let left_sources: Vec<_> = left_entity
            .sources
            .iter()
            .filter_map(|id| merged.database().sources.get(*id))
            .collect();

        let right_sources: Vec<_> = right_entity
            .sources
            .iter()
            .filter_map(|id| merged.database().sources.get(*id))
            .collect();

        assert_eq!(left_sources, vec!["open food facts"]);
        assert_eq!(right_sources, vec!["manufacturer registry"]);

        assert_eq!(merged.database().tags.len(), 2);
        assert_eq!(merged.database().sources.len(), 2);

        Ok(())
    }

    fn create_database_with_metadata(
        parent: &Path,
        name: &str,
        id: usize,
        alias: &str,
        tags: Vec<&str>,
        sources: Vec<&str>,
    ) -> Result<std::path::PathBuf> {
        let mut engine = Engine::create(parent, name)?;

        engine.insert(EntityInput {
            id: EntityID::new(id),
            aliases: vec![alias.into()],
            tags: tags.into_iter().map(Into::into).collect(),
            sources: sources
                .into_iter()
                .map(|provider| eir_core::entity::input::SourceInput {
                    provider: provider.into(),
                    verified: false,
                    created: None,
                    updated: None,
                })
                .collect(),
            attributes: vec![],
            relationships: vec![],
        })?;

        engine.flush()?;

        Ok(parent.join(name).join(format!("{name}.eir")))
    }

    #[test]
    fn merge_remaps_attribute_keys_and_relationship_types() -> Result<()> {
        let temp = tempdir()?;

        let left = create_database_with_attributes_and_relationships(
            temp.path(),
            "left",
            1000,
            "Left Entity",
            "brand",
            "Acme",
            "manufacturer",
            9000,
        )?;

        let right = create_database_with_attributes_and_relationships(
            temp.path(),
            "right",
            2000,
            "Right Entity",
            "country",
            "Denmark",
            "origin",
            9001,
        )?;

        let output = temp.path().join("merged").join("merged.eir");

        println!(
            "left attribute keys: {:?}",
            Engine::open(&left)?
                .database()
                .attribute_keys
                .iter()
                .collect::<Vec<_>>()
        );

        let report = merge(&left, &right, &output)?;

        assert_eq!(report.entities_added, 2);
        assert_eq!(report.entities_skipped, 0);

        let merged = Engine::open(&output)?;

        assert_eq!(merged.database().attribute_keys.len(), 2);
        assert_eq!(merged.database().relationship_types.len(), 2);

        let left_entity = merged.entity(EntityID::new(1000)).unwrap();
        let right_entity = merged.entity(EntityID::new(2000)).unwrap();

        // Attribute keys must resolve to their logical names after merging.
        let left_attribute_keys: Vec<_> = left_entity
            .attributes
            .iter()
            .filter_map(|attribute| merged.database().attribute_keys.get(attribute.key))
            .collect();

        let right_attribute_keys: Vec<_> = right_entity
            .attributes
            .iter()
            .filter_map(|attribute| merged.database().attribute_keys.get(attribute.key))
            .collect();

        assert_eq!(left_attribute_keys, vec!["brand"]);
        assert_eq!(right_attribute_keys, vec!["country"]);

        let left_relationship_types: Vec<_> = left_entity
            .relationships
            .iter()
            .filter_map(|relationship| match relationship.kind {
                RelationshipType::Custom(id) => merged.database().relationship_types.get(id),
                _ => None,
            })
            .collect();

        let right_relationship_types: Vec<_> = right_entity
            .relationships
            .iter()
            .filter_map(|relationship| match relationship.kind {
                RelationshipType::Custom(id) => merged.database().relationship_types.get(id),
                _ => None,
            })
            .collect();

        assert_eq!(left_relationship_types, vec!["manufacturer"]);
        assert_eq!(right_relationship_types, vec!["origin"]);

        Ok(())
    }

    fn create_database_with_attributes_and_relationships(
        parent: &Path,
        name: &str,
        id: usize,
        alias: &str,
        attribute_key: &str,
        attribute_value: &str,
        relationship_kind: &str,
        relationship_target: usize,
    ) -> Result<std::path::PathBuf> {
        use eir_core::entity::input::{AttributesInput, RelationshipInput};

        let mut engine = Engine::create(parent, name)?;

        engine.insert(EntityInput {
            id: EntityID::new(id),
            aliases: vec![alias.into()],
            tags: vec![],
            sources: vec![],
            attributes: vec![AttributesInput {
                key: attribute_key.into(),
                value: attribute_value.into(),
            }],
            relationships: vec![RelationshipInput {
                kind: relationship_kind.into(),
                target: EntityID::new(relationship_target),
            }],
        })?;

        engine.flush()?;

        Ok(parent.join(name).join(format!("{name}.eir")))
    }
}
