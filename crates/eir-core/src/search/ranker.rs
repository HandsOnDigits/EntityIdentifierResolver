use std::collections::{HashMap, HashSet};

use crate::{
    entity::prelude::types::EntityID,
    search::result::{SearchExplanation, SearchHit, SearchSource},
};

#[derive(Default, Clone)]
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
        let strongest = sources
            .iter()
            .map(|s| Self::weight(*s))
            .max_by(|a, b| a.total_cmp(b))
            .unwrap_or(0.0);

        let supporting = (sources.len().saturating_sub(1) as f32) * 0.03;

        (strongest + supporting).min(1.0)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::prelude::types::TagID;

    fn exact(alias: &str) -> SearchExplanation {
        SearchExplanation::ExactAlias {
            alias: alias.into(),
        }
    }

    fn fuzzy(alias: &str) -> SearchExplanation {
        SearchExplanation::FuzzyAlias {
            alias: alias.into(),
        }
    }

    fn token(token: &str) -> SearchExplanation {
        SearchExplanation::Token {
            token: token.into(),
        }
    }

    #[test]
    fn exact_alias_beats_fuzzy() {
        let ranker = Ranker::new();

        let results = ranker.rank(vec![
            (EntityID(1), SearchSource::FuzzyAlias, fuzzy("Appl")),
            (EntityID(2), SearchSource::ExactAlias, exact("Apple")),
        ]);

        assert_eq!(results[0].entity_id, EntityID(2));
        assert_eq!(results[0].score, 1.0);
    }

    #[test]
    fn multiple_signals_add_bonus() {
        let ranker = Ranker::new();

        let results = ranker.rank(vec![
            (EntityID(1), SearchSource::Token, token("apple")),
            (
                EntityID(1),
                SearchSource::Tag,
                SearchExplanation::Tag { tag: TagID(1) },
            ),
        ]);

        assert_eq!(results.len(), 1);
        assert!(results[0].score > 0.40);
    }

    #[test]
    fn attribute_key_value_beats_attribute_value() {
        let candidates = vec![
            (
                EntityID(1),
                SearchSource::AttributeValue,
                SearchExplanation::AttributeValue {
                    term: "milk".into(),
                },
            ),
            (
                EntityID(2),
                SearchSource::AttributeKeyValue,
                SearchExplanation::AttributeKeyValue {
                    key: "brand".into(),
                    value: "nestle".into(),
                },
            ),
        ];

        let results = Ranker::new().rank(candidates);

        assert_eq!(results[0].entity_id, EntityID(2));
    }
}
