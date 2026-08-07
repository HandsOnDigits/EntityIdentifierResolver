use std::collections::HashMap;

use crate::entity::prelude::types::EntityID;

use super::Signal;

pub type SignalSet = Vec<Signal>;

#[derive(Default)]
pub struct CandidateSet {
    entities: HashMap<EntityID, Candidate>,
}

impl CandidateSet {
    pub fn get(&self, entity: EntityID) -> Option<&Candidate> {
        self.entities.get(&entity)
    }
}

pub struct Candidate {
    pub entity: EntityID,
    pub signals: SignalSet,
}

impl CandidateSet {
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
}
