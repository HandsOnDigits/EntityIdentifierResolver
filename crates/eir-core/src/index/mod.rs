mod alias;
mod bk_tree;
mod inverted;
mod ranker;
mod resolver;
mod trie;
pub mod utils;

pub use alias::{AliasIndex, AliasIndexRecord};
pub use bk_tree::{BKTreeIndex, BKTreeIndexRecord};
pub use inverted::{InvertedIndex, InvertedIndexRecord};
pub use ranker::{Ranker, SearchHit};
pub use resolver::{Resolver, SearchResult};
pub use trie::{TrieIndex, TrieIndexRecord};

pub mod prelude {
    pub use super::{
        AliasIndex, AliasIndexRecord, BKTreeIndex, BKTreeIndexRecord, InvertedIndex,
        InvertedIndexRecord, TrieIndex, TrieIndexRecord,
    };
}
