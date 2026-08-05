use std::collections::{HashMap, HashSet};

use super::search::{SearchHit, SearchSource};
use crate::entity::types::EntityID;

#[derive(Default)]
pub struct Ranker;

impl Ranker {
    pub fn new() -> Self {
        Self
    }

    fn weight(source: SearchSource) -> f32 {
        match source {
            SearchSource::ExactAlias => 1.0,
            SearchSource::PrefixAlias => 0.75,
            SearchSource::FuzzyAlias => 0.60,
            SearchSource::Token => 0.40,
            SearchSource::Relationship => 0.50,
            SearchSource::Tag => 0.30,
            SearchSource::Property => 0.20,
            SearchSource::Source => 0.10,
        }
    }

    fn calculate_score(sources: &HashSet<SearchSource>) -> f32 {
        // Exact match dominates.
        if sources.contains(&SearchSource::ExactAlias) {
            return 1.0;
        }

        let mut score = sources
            .iter()
            .map(|source| Self::weight(*source))
            .fold(0.0, f32::max);

        // Multiple independent signals increase confidence.
        if sources.len() > 1 {
            score += 0.05;
        }

        score.min(1.0)
    }

    pub fn rank(&self, candidates: Vec<(EntityID, SearchSource)>) -> Vec<SearchHit> {
        let mut merged: HashMap<EntityID, HashSet<SearchSource>> = HashMap::new();

        for (entity_id, source) in candidates {
            merged.entry(entity_id).or_default().insert(source);
        }

        let mut results: Vec<SearchHit> = merged
            .into_iter()
            .map(|(entity_id, sources)| {
                let score = Self::calculate_score(&sources);

                SearchHit {
                    entity_id,
                    score,
                    sources,
                }
            })
            .collect();

        results.sort_by(|a, b| b.score.total_cmp(&a.score));

        results
    }
}
