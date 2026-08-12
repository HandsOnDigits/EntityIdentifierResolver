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

    use eir_core::entity::{input::EntityInput, prelude::types::EntityID};

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
}
