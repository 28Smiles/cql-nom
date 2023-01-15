/// Definition of the possible types of the CQL data model.
pub mod cql_type;
/// Definition of an identifier.
pub mod identifier;
/// Definition of order.
pub mod order;
/// Definition of an identifier with a possible keyspace.
pub mod qualified_identifier;
/// Definition of a statement.
pub mod statement;
/// Definition of a table.
pub mod table;
/// Definition of a user defined type.
pub mod user_defined_type;

pub use cql_type::*;
pub use identifier::*;
pub use order::*;
pub use qualified_identifier::*;
pub use statement::*;
pub use table::*;
pub use user_defined_type::*;

/// A tree node with an identifier.
pub trait Identifiable<I: Clone> {
    /// The keyspace of the identifier.
    fn keyspace(&self) -> Option<&CqlIdentifier<I>>;
    /// The name of the identifier.
    fn identifier(&self) -> &CqlIdentifier<I>;
    /// The active keyspace based on the context.
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
    /// The active identifier based on the context.
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
