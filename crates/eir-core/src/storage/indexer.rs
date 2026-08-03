use crate::{
    entity::{
        EntityInput,
        types::{SourceID, TagID},
    },
    storage::{Indexes, posting_list::PostingList},
};

use super::database::ArchivedPostingList;

use rkyv::{Archive, Deserialize, Serialize};

#[derive(Archive, Serialize, Deserialize)]
pub struct ArchivedIndexes {
    pub tags: ArchivedPostingList<TagID>,
    pub sources: ArchivedPostingList<SourceID>,
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
