use std::hash::Hash;

pub trait RegistryID: Copy + Eq + Hash {
    fn new(index: usize) -> Self;
    fn index(self) -> usize;
}

macro_rules! define_id {
    (
        $(#[$meta:meta])*
        $name:ident
    ) => {
        #[derive(
            Debug,
            Clone,
            Copy,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Hash,
            rkyv::Archive,
            rkyv::Serialize,
            rkyv::Deserialize,
            serde::Serialize,
            serde::Deserialize,
        )]
        #[serde(transparent)]
        pub struct $name {
            index: usize,
        }

        impl $name {
            pub const fn new(index: usize) -> Self {
                Self { index }
            }

            pub const fn index(self) -> usize {
                self.index
            }
        }

        impl RegistryID for $name {
            fn new(index: usize) -> Self {
                Self { index }
            }

            fn index(self) -> usize {
                self.index
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.index)
            }
        }

        impl std::str::FromStr for $name {
            type Err = std::num::ParseIntError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let index = s.parse::<usize>()?;
                Ok(Self { index })
            }
        }
    };
}

define_id! {
    /// Identifies an entity within a database.
    ///
    /// Entity IDs must be unique within a database. Database insertion
    /// rejects an entity when another entity already has the same ID.
    EntityID
}

define_id! {
    /// Identifies a tag in the database tag registry.
    TagID
}

define_id! {
    /// Identifies a source in the database source registry.
    SourceID
}

define_id! {
    /// Identifies an attribute key in the attribute-key registry.
    AttributeKeyID
}

define_id! {
    /// Identifies a relationship type in the relationship-type registry.
    RelationshipTypeID
}

// Yha, It may be a bit of an anit patten to use macros for this,
// but at least, I don't forget to add the right drive.
