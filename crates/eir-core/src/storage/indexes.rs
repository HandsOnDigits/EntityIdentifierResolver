use rkyv::{Archive, Deserialize, Serialize};

use crate::entity::{
    EntityInput,
    types::{SourceID, TagID},
};

use super::posting_list::{PostingList, PostingListRecord};

#[derive(Default)]
pub struct Indexes {
    pub tags: PostingList<TagID>,
    pub sources: PostingList<SourceID>,
}

#[derive(Archive, Serialize, Deserialize)]
pub struct ArchivedIndexes {
    pub tags: PostingListRecord<TagID>,
    pub sources: PostingListRecord<SourceID>,
}

pub struct IndexBuilder;

impl IndexBuilder {
    pub fn build(inputs: &[EntityInput]) -> Indexes {
        let mut tags = PostingList::default();
        let mut sources = PostingList::default();

        for input in inputs {
            let entity_id = input.document.id;

            for tag in &input.tags {
                tags.insert(*tag, entity_id);
            }

            for source in &input.document.sources {
                sources.insert(*source, entity_id);
            }
        }

        Indexes { tags, sources }
    }
}
