use std::collections::HashMap;

use crate::entity::prelude::types::EntityID;

use super::Signal;

pub type SignalSet = Vec<Signal>;

#[derive(Default)]
pub struct CandidateSet {
    entities: HashMap<EntityID, Candidate>,
}

pub struct Candidate {
    pub entity: EntityID,
    pub signals: SignalSet,
}

impl CandidateSet {
    pub fn get(&self, entity: EntityID) -> Option<&Candidate> {
        self.entities.get(&entity)
    }

    pub fn add_signal(&mut self, entity: EntityID, signal: Signal) {
        self.entities
            .entry(entity)
            .or_insert_with(|| Candidate {
                entity,
                signals: Vec::new(),
            })
            .signals
            .push(signal);
    }

    pub fn union(mut self, other: Self) -> Self {
        for candidate in other.entities.into_values() {
            let entry = self.entities.entry(candidate.entity).or_insert(Candidate {
                entity: candidate.entity,
                signals: Vec::new(),
            });

            entry.signals.extend(candidate.signals);
        }

        self
    }

    pub fn intersection(self, other: &Self) -> Self {
        let mut result = CandidateSet::default();

        for (entity, candidate) in self.entities {
            let Some(other_candidate) = other.entities.get(&entity) else {
                continue;
            };

            let mut signals = candidate.signals;
            signals.extend(other_candidate.signals.clone());

            result
                .entities
                .insert(entity, Candidate { entity, signals });
        }

        result
    }

    pub fn union_with(&mut self, other: CandidateSet) {
        for (entity, candidate) in other.entities {
            for signal in candidate.signals {
                self.add_signal(entity, signal);
            }
        }
    }

    pub fn intersect_with(&mut self, other: &CandidateSet) {
        self.entities
            .retain(|entity, _| other.entities.contains_key(entity));
    }

    pub fn contains(&self, entity: EntityID) -> bool {
        self.entities.contains_key(&entity)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Candidate> {
        self.entities.values()
    }

    pub fn len(&self) -> usize {
        self.entities.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn union_contains_candidates_from_both_sets() {
        let mut left = CandidateSet::default();
        left.add_signal(EntityID::new(1), Signal::Property);

        let mut right = CandidateSet::default();
        right.add_signal(EntityID::new(2), Signal::Relationship);

        let result = left.union(right);

        assert!(result.contains(EntityID::new(1)));
        assert!(result.contains(EntityID::new(2)));
    }

    #[test]
    fn intersection_only_contains_candidates_in_both_sets() {
        let mut left = CandidateSet::default();
        left.add_signal(EntityID::new(1), Signal::Property);
        left.add_signal(EntityID::new(2), Signal::Property);

        let mut right = CandidateSet::default();
        right.add_signal(EntityID::new(2), Signal::Relationship);
        right.add_signal(EntityID::new(3), Signal::Relationship);

        let result = left.intersection(&right);

        assert!(!result.contains(EntityID::new(1)));
        assert!(result.contains(EntityID::new(2)));
        assert!(!result.contains(EntityID::new(3)));

        let candidate = result.get(EntityID::new(2)).unwrap();

        assert!(candidate.signals.contains(&Signal::Property));
        assert!(candidate.signals.contains(&Signal::Relationship));
    }
}
