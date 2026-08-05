use std::collections::HashMap;

use eir_core::{Database, storage::IndexBuilder};

use super::{context::BuilderContext, loader::load_entities, mapper, writer::write_database};

pub fn build(
    input: impl AsRef<std::path::Path>,
    output: impl AsRef<std::path::Path>,
) -> anyhow::Result<()> {
    let mut ctx = BuilderContext::new();

    let fixtures = load_entities(input)?;

    let inputs = fixtures
        .into_iter()
        .map(|entity| mapper::map(entity, &mut ctx))
        .collect::<Vec<_>>();

    let indexes = IndexBuilder::build(&inputs);

    let mut aliases = HashMap::new();
    let mut entities = Vec::new();

    for input in inputs {
        aliases.insert(input.id, input.aliases.clone());

        entities.push(input);
    }

    let database = Database {
        entities,

        tags: ctx.tags.into_inner(),
        sources: ctx.sources.into_inner(),
        attribute_keys: ctx.attribute_keys.into_inner(),

        alias_index: indexes.alias.to_record(),
        trie_index: indexes.trie.to_record(),
        bk_tree_index: indexes.bk_tree.to_record(),
        inverted_index: indexes.inverted.to_record(),

        tag_index: indexes.tags.to_record(),
        source_index: indexes.sources.to_record(),
    };

    write_database(database, output)?;

    Ok(())
}
