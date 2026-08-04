use rkyv::{Archive, Deserialize, Serialize};

use crate::entity::{
    EntityDocument,
    types::{SourceID, TagID},
};

use crate::index::{prelude::*, utils::normalize};

use super::posting_list::{PostingList, PostingListRecord};

#[derive(Default, Debug)]
pub struct Indexes {
    pub alias: AliasIndex,
    pub trie: TrieIndex,
    pub bk_tree: BKTreeIndex,
    pub inverted: InvertedIndex,

    pub tags: PostingList<TagID>,
    pub sources: PostingList<SourceID>,
}

#[derive(Archive, Serialize, Deserialize, Debug)]
pub struct ArchivedIndexes {
    pub tags: PostingListRecord<TagID>,
    pub sources: PostingListRecord<SourceID>,
}

pub struct IndexBuilder;

impl IndexBuilder {
    pub fn build(inputs: &[EntityDocument]) -> Indexes {
        let mut indexes = Indexes::default();

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
        }

        indexes
    }
}
