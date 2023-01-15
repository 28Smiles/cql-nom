use crate::model::*;
use derive_new::new;
use getset::Getters;
use std::ops::Deref;

/// A identifier with a possible keyspace prefix.
#[derive(Debug, Clone, new, Getters)]
pub struct CqlQualifiedIdentifier<I> {
    /// The keyspace of the identifier.
    #[getset(get = "pub")]
    keyspace: Option<CqlIdentifier<I>>,
    /// The name of the identifier.
    #[getset(get = "pub")]
    name: CqlIdentifier<I>,
}

impl<I: Deref<Target = str>> PartialEq for CqlQualifiedIdentifier<I> {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.keyspace == other.keyspace && self.name == other.name
    }
}

impl<I: Clone + Deref<Target = str>> Identifiable<I> for CqlQualifiedIdentifier<I> {
    #[inline(always)]
    fn keyspace(&self) -> Option<&CqlIdentifier<I>> {
        self.keyspace.as_ref()
    }

    #[inline(always)]
    fn identifier(&self) -> &CqlIdentifier<I> {
        &self.name
    }
}
