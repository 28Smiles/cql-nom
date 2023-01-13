use crate::model::identifier::CqlIdentifier;
use crate::model::Identifiable;
use derive_new::new;
use getset::Getters;
use std::ops::Deref;

#[derive(Debug, Clone, new, Getters)]
pub struct CqlQualifiedIdentifier<I> {
    /// The keyspace of the identifier.
    keyspace: Option<CqlIdentifier<I>>,
    /// The name of the identifier.
    #[get = "pub"]
    name: CqlIdentifier<I>,
}

impl<I> CqlQualifiedIdentifier<I> {
    #[inline(always)]
    pub fn keyspace(&self) -> Option<&CqlIdentifier<I>> {
        self.keyspace.as_ref()
    }
}

impl<I: Deref<Target = str>> PartialEq for CqlQualifiedIdentifier<I> {
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
