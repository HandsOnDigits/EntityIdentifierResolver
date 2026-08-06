#[derive(Debug, Clone)]
pub enum Filter {
    Attribute { key: Box<str>, value: Box<str> },

    Tag { tag: Box<str> },

    Source { source: Box<str> },

    Relationship { kind: Box<str>, target: Box<str> },

    EntityType { kind: Box<str> },
}
