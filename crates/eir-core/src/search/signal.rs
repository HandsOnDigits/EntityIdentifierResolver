#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Signal {
    ExactAlias,
    PrefixAlias,
    FuzzyAlias { distance: u8 },
    Token { matches: u16 },
    Tag,
    Property,
    Relationship,
}

pub type SignalSet = Vec<Signal>;
