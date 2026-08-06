pub enum SearchStage {
    ExactAlias,
    PrefixAlias,
    FuzzyAlias,
    Token,
    Tag,
    Property,
    Relationship,
}

pub struct SearchPlan {
    pub stages: Vec<SearchStage>,
}
