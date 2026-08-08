use std::hash::Hash;

pub trait RegistryID: Copy + Eq + Hash {
    fn new(index: usize) -> Self;
    fn index(self) -> usize;
}

macro_rules! define_id {
    ($name:ident) => {
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

define_id!(EntityID);
define_id!(TagID);
define_id!(SourceID);
define_id!(AttributeKeyID);
define_id!(RelationshipTypeID);
