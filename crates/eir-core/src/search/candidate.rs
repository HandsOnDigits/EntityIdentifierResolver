use crate::entity::types::EntityID;

use std::collections::HashMap;

use super::signal::SignalSet;

pub struct CandidateSet {
    entities: HashMap<EntityID, Candidate>,
}

pub struct Candidate {
    pub entity: EntityID,
    pub signals: SignalSet,
}
