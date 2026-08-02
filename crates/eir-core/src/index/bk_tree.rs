use crate::entity::types::EntityID;

#[derive(Default)]
pub struct BKTreeIndex {
    entries: Vec<(String, EntityID)>,
}

impl BKTreeIndex {
    pub fn insert(&mut self, text: &str, entity_id: EntityID) {
        self.entries.push((text.to_lowercase(), entity_id));
    }

    pub fn search(&self, query: &str, distance: usize) -> Vec<EntityID> {
        self.entries
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
