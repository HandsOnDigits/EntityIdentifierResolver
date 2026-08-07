#[macro_export]
macro_rules! archived_id_index {
    ($id:expr) => {
        $id.0.to_native() as usize
    };
}

#[macro_export]
macro_rules! define_id {
    ($name:ident, $inner:ty) => {
        #[derive(
            Debug,
            Clone,
            Copy,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Hash,
            Default,
            serde::Serialize,
            serde::Deserialize,
            rkyv::Archive,
            rkyv::Serialize,
            rkyv::Deserialize,
        )]
        #[rkyv(derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord))]
        #[repr(transparent)]
        pub struct $name(pub $inner);

        impl $name {
            #[inline]
            pub const fn new(value: $inner) -> Self {
                Self(value)
            }

            #[inline]
            pub const fn as_inner(self) -> $inner {
                self.0
            }

            #[inline]
            pub const fn as_usize(self) -> usize {
                self.0 as usize
            }

            #[inline]
            pub const fn from_usize(value: usize) -> Self {
                Self(value as $inner)
            }
        }

        impl From<$inner> for $name {
            #[inline]
            fn from(value: $inner) -> Self {
                Self(value)
            }
        }

        impl From<$name> for $inner {
            #[inline]
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl std::str::FromStr for $name {
            type Err = <$inner as std::str::FromStr>::Err;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(Self(s.parse()?))
            }
        }

        impl PartialEq<$name> for rkyv::Archived<$name> {
            #[inline]
            fn eq(&self, other: &$name) -> bool {
                self.0 == other.0
            }
        }

        impl PartialEq<rkyv::Archived<$name>> for $name {
            #[inline]
            fn eq(&self, other: &rkyv::Archived<$name>) -> bool {
                self.0 == other.0
            }
        }
    };
}

#[macro_export]
macro_rules! define_registry_id {
    ($name:ident, $inner:ty) => {
        impl $crate::storage::registry::RegistryID for $name {
            #[inline]
            fn from_index(index: usize) -> Self {
                Self(index as $inner)
            }

            #[inline]
            fn index(self) -> usize {
                self.0 as usize
            }
        }
    };
}

pub use archived_id_index;
pub use define_id;
pub use define_registry_id;
