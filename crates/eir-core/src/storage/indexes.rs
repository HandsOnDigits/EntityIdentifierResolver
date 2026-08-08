use rkyv::{Archive, Deserialize, Serialize};

use crate::{
    entity::prelude::{
        EntityDocument,
        types::{EntityID, RelationshipTypeID, SourceID, TagID},
    },
    storage::Registry,
};

use crate::{index::prelude::*, utils::normalize};

use super::posting_list::{PostingList, PostingListRecord};

#[derive(Default, Debug)]
pub struct Indexes {
    pub alias: AliasIndex,
    pub trie: TrieIndex,
    pub bk_tree: BKTreeIndex,
    pub inverted: InvertedIndex,

    pub tags: PostingList<TagID>,
    pub sources: PostingList<SourceID>,

    pub attribute_keys: InvertedIndex,
    pub attribute_values: InvertedIndex,
    pub attribute_pairs: InvertedIndex,

    pub relationships: PostingList<EntityID>,
    pub relationships_type: Registry<RelationshipTypeID>,
}

#[derive(Archive, Serialize, Deserialize, Debug)]
pub struct ArchivedIndexes {
    pub tags: PostingListRecord<TagID>,
    pub sources: PostingListRecord<SourceID>,
}

pub struct IndexBuilder;

impl IndexBuilder {
    pub fn build<'a>(
        inputs: &[EntityDocument],
        attribute_keys: impl IntoIterator<Item = &'a Box<str>>,
    ) -> Indexes {
        let mut indexes = Indexes::default();

        // Collect keys into a contiguous slice for O(1) indexed access via attribute.key.index()
        let keys: Vec<&str> = attribute_keys.into_iter().map(|k| k.as_ref()).collect();

        for entity in inputs {
            let id = entity.id;

            for alias in &entity.aliases {
                let normalized = normalize(alias);

                indexes.alias.insert(normalized.clone(), id);
                indexes.trie.insert(&normalized, id);
                indexes.bk_tree.insert(&normalized, id);

                for token in normalized.split_whitespace() {
                    indexes.inverted.insert(token, id);
                }
            }

            for tag in &entity.tags {
                indexes.tags.insert(*tag, id);
            }

            for source in &entity.sources {
                indexes.sources.insert(*source, id);
            }

            for attribute in &entity.attributes {
                let Some(&key_str) = keys.get(attribute.key.index()) else {
                    continue;
                };

                let key = normalize(key_str);
                let value = attribute.value.normalized();

                indexes.attribute_keys.insert(&key, id);

                indexes.attribute_values.insert(&value, id);

                indexes
                    .attribute_pairs
                    .insert(&format!("{key}:{value}"), id);

                for token in value.split_whitespace() {
                    indexes.attribute_values.insert(token, id);
                }
            }

            for relationship in &entity.relationships {
                indexes.relationships.insert(relationship.target, entity.id);
            }
        }

        indexes
    }
}
