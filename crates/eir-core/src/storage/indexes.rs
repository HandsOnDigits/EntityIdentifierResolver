use super::posting_list::PostingList;
use crate::entity::types::{SourceID, TagID};

#[derive(Default)]
pub struct Indexes {
    pub tags: PostingList<TagID>,
    pub sources: PostingList<SourceID>,
}
