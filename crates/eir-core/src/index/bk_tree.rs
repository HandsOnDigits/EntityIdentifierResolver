use bytecheck::CheckBytes;

use rkyv::{Archive, Deserialize, Serialize};

use crate::entity::types::EntityID;

#[derive(Default, Debug)]
pub struct BKTreeIndex {
    root: Vec<(Box<str>, EntityID)>,
}

use super::utils::normalize;

impl BKTreeIndex {
    pub fn insert(&mut self, text: &str, entity_id: EntityID) {
        self.root.push((normalize(text), entity_id));
    }

    pub fn search(&self, query: &str, distance: usize) -> Vec<EntityID> {
        self.root
            .iter()
            .filter_map(|(text, id)| {
                if levenshtein(text, query) <= distance {
                    Some(*id)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn export(&self) -> Vec<(Box<str>, Vec<EntityID>)> {
        self.root
            .iter()
            .map(|(key, entity)| (key.clone(), vec![*entity]))
            .collect()
    }
}

fn levenshtein(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();

    let mut prev: Vec<usize> = (0..=b.len()).collect();
    let mut curr = vec![0; b.len() + 1];

    for (i, ca) in a.iter().enumerate() {
        curr[0] = i + 1;

        for (j, cb) in b.iter().enumerate() {
            let cost = if ca == cb { 0 } else { 1 };

            curr[j + 1] =
                std::cmp::min(std::cmp::min(curr[j] + 1, prev[j + 1] + 1), prev[j] + cost);
        }

        std::mem::swap(&mut prev, &mut curr);
    }

    prev[b.len()]
}

#[derive(Archive, Serialize, Deserialize, Debug, Clone, CheckBytes, Default)]
pub struct BKTreeIndexRecord {
    pub entries: Vec<(Box<str>, Vec<EntityID>)>,
}

impl BKTreeIndex {
    pub fn to_record(&self) -> BKTreeIndexRecord {
        BKTreeIndexRecord {
            entries: self.export(),
        }
    }

    pub fn from_record(record: BKTreeIndexRecord) -> Self {
        let mut trie = Self::default();

        for (word, ids) in record.entries {
            for id in ids {
                trie.insert(&word, id);
            }
        }

        trie
    }
}
