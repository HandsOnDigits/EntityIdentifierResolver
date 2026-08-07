use std::collections::{HashMap, HashSet};

use crate::{
    entity::prelude::types::EntityID,
    search::result::{SearchExplanation, SearchHit, SearchSource},
};

use super::signal::SignalSet;

pub struct RankedCandidate {
    pub entity: EntityID,
    pub score: f32,
    pub signals: SignalSet,
}

#[derive(Default)]
pub struct Ranker;

impl Ranker {
    pub fn new() -> Self {
        Self
    }

    fn weight(source: SearchSource) -> f32 {
        match source {
            SearchSource::ExactAlias => 1.00,

            // name similarity
            SearchSource::PrefixAlias => 0.75,
            SearchSource::FuzzyAlias => 0.60,
            SearchSource::Token => 0.40,

            // graph signals
            SearchSource::Relationship => 0.70,

            // metadata
            SearchSource::Tag => 0.25,
            SearchSource::AttributeKey => 0.20,
            SearchSource::AttributeValue => 0.50,
            SearchSource::AttributeKeyValue => 0.85,

            // provenance
            SearchSource::Source => 0.45,
        }
    }

    fn calculate_score(sources: &HashSet<SearchSource>) -> f32 {
        let base = sources
            .iter()
            .map(|source| Self::weight(*source))
            .fold(0.0, f32::max);

        let bonus = match sources.len() {
            0 | 1 => 0.0,
            2 => 0.05,
            3 => 0.10,
            _ => 0.15,
        };

        (base + bonus).min(1.0)
    }

    pub fn rank(
        &self,
        candidates: Vec<(EntityID, SearchSource, SearchExplanation)>,
    ) -> Vec<SearchHit> {
        let mut merged: HashMap<EntityID, SearchHit> = HashMap::new();

        for (entity_id, source, explanation) in candidates {
            let entry = merged.entry(entity_id).or_insert_with(|| SearchHit {
                entity_id,
                score: 0.0,
                sources: HashSet::new(),
                explanations: HashSet::new(),
            });

            entry.sources.insert(source);
            entry.explanations.insert(explanation);
        }

        let mut results: Vec<SearchHit> = merged
            .into_values()
            .map(|mut hit| {
                hit.score = Self::calculate_score(&hit.sources);
                hit
            })
            .collect();

        results.sort_by(|a, b| b.score.total_cmp(&a.score));

        results
    }
}
