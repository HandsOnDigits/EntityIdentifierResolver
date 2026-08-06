#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Signal {
    ExactAlias,
    PrefixAlias,
    FuzzyAlias,
    Token,
    Tag,
    Property,
    Relationship,
}

pub type SignalSet = Vec<Signal>;
