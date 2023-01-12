use crate::model::identifier::CqlIdentifier;
use crate::model::qualified_identifier::CqlQualifiedIdentifier;

pub mod cql_type;
pub mod identifier;
pub mod order;
pub mod qualified_identifier;
pub mod statement;
pub mod table;
pub mod user_defined_type;

pub trait Identifiable<I> {
    fn identifier(&self, keyspace: Option<&CqlIdentifier<I>>) -> CqlQualifiedIdentifier<I>;
}
