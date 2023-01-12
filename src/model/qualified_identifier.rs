use crate::model::identifier::CqlIdentifier;
use crate::model::Identifiable;
use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct CqlQualifiedIdentifier<I> {
    keyspace: Option<CqlIdentifier<I>>,
    name: CqlIdentifier<I>,
}

impl<I> CqlQualifiedIdentifier<I> {
    pub fn new(keyspace: Option<CqlIdentifier<I>>, name: CqlIdentifier<I>) -> Self {
        CqlQualifiedIdentifier { keyspace, name }
    }

    pub fn keyspace(&self) -> Option<&CqlIdentifier<I>> {
        self.keyspace.as_ref()
    }

    pub fn name(&self) -> &CqlIdentifier<I> {
        &self.name
    }
}

impl<I: Deref<Target = str>> PartialEq for CqlQualifiedIdentifier<I> {
    fn eq(&self, other: &Self) -> bool {
        self.keyspace == other.keyspace && self.name == other.name
    }
}

impl<I: Clone + Deref<Target = str>> Identifiable<I> for CqlQualifiedIdentifier<I> {
    fn identifier(&self, keyspace: Option<&CqlIdentifier<I>>) -> CqlQualifiedIdentifier<I> {
        if let Some(keyspace) = &self.keyspace {
            // The identifier already has a keyspace.
            CqlQualifiedIdentifier::new(Some(keyspace.clone()), self.name.clone())
        } else {
            // The identifier does not have a keyspace.
            CqlQualifiedIdentifier::new(keyspace.map(Clone::clone), self.name.clone())
        }
    }
}