use crate::model::identifier::CqlIdentifier;
use crate::model::qualified_identifier::CqlQualifiedIdentifier;

pub mod cql_type;
pub mod identifier;
pub mod order;
pub mod qualified_identifier;
pub mod statement;
pub mod table;
pub mod user_defined_type;

pub trait Identifiable<I: Clone> {
    fn keyspace(&self) -> Option<&CqlIdentifier<I>>;
    fn identifier(&self) -> &CqlIdentifier<I>;
    fn contextualized_keyspace(
        &self,
        keyspace: Option<&CqlIdentifier<I>>,
    ) -> Option<CqlIdentifier<I>> {
        if let Some(keyspace) = self.keyspace() {
            // The identifier already has a keyspace.
            Some(keyspace.clone())
        } else {
            // The identifier does not have a keyspace.
            keyspace.cloned()
        }
    }
    fn contextualized_identifier(
        &self,
        keyspace: Option<&CqlIdentifier<I>>,
    ) -> CqlQualifiedIdentifier<I> {
        CqlQualifiedIdentifier::new(
            self.contextualized_keyspace(keyspace),
            self.identifier().clone(),
        )
    }
}
