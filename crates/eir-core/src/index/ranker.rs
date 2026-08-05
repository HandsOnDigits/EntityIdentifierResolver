use std::collections::HashMap;

use super::resolver::SearchSource;
use crate::entity::types::EntityID;

#[derive(Debug, Clone)]
pub struct SearchHit {
    pub entity_id: EntityID,
    pub score: f32,
    pub sources: Vec<SearchSource>,
}

#[derive(Default)]
pub struct Ranker;

impl Ranker {
    pub fn new() -> Self {
        Self
    }

    pub fn rank(&self, candidates: Vec<(EntityID, f32, SearchSource)>) -> Vec<SearchHit> {
        let mut merged: HashMap<EntityID, SearchHit> = HashMap::new();

        for (entity_id, score, source) in candidates {
            merged
                .entry(entity_id)
                .and_modify(|hit| {
                    hit.score += score;
                    hit.sources.push(source);
                })
                .or_insert(SearchHit {
                    entity_id,
                    score,
                    sources: vec![source],
                });
        }

        let mut results: Vec<SearchHit> = merged.into_values().collect();

        results.sort_by(|a, b| b.score.total_cmp(&a.score));

        results
    }
}
