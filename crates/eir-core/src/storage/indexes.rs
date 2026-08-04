use rkyv::{Archive, Deserialize, Serialize};

use crate::entity::{
    EntityDocument,
    types::{SourceID, TagID},
};

use super::posting_list::{PostingList, PostingListRecord};

#[derive(Default, Debug)]
pub struct Indexes {
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
        let mut tags = PostingList::<TagID>::default();
        let mut sources = PostingList::<SourceID>::default();

        for input in inputs {
            let entity_id = input.id;

            for tag in &input.tags {
                tags.insert(*tag, entity_id);
            }

            for source in &input.sources {
                sources.insert(*source, entity_id);
            }
        }

        Indexes { tags, sources }
    }
}
