use crate::model::cql_type::CqlType;
use crate::model::identifier::CqlIdentifier;
use crate::model::qualified_identifier::CqlQualifiedIdentifier;
use derive_where::derive_where;

/// User-defined type.
/// More Information: <https://cassandra.apache.org/doc/latest/cassandra/cql/types.html#user-defined-types>
///
/// Type Grammar:
/// ```bnf
/// user_defined_type::= udt_name
/// udt_name::= [ keyspace_name '.' ] identifier
/// ```
///
/// Definition Grammar:
/// ```bnf
/// create_type_statement::= CREATE TYPE [ IF NOT EXISTS ] udt_name
///         '(' field_definition ( ',' field_definition)* ')'
/// field_definition::= identifier cql_type
/// ```
///
/// Example Definition:
/// ```cql
/// CREATE TYPE IF NOT EXISTS user (
///    id uuid,
///    name text,
///    age int
/// );
/// ```
#[derive(Debug, Clone)]
#[derive_where(PartialEq; UdtType, I: std::ops::Deref<Target = str> + std::cmp::PartialEq)]
pub struct CqlUserDefinedType<I, UdtType> {
    if_not_exists: bool,
    /// The name of the user-defined type.
    name: CqlQualifiedIdentifier<I>,
    /// The fields of the user-defined type.
    fields: Vec<(CqlIdentifier<I>, CqlType<UdtType>)>,
}

impl<I, UdtType> CqlUserDefinedType<I, UdtType> {
    /// Creates a new user-defined type.
    pub fn new(
        if_not_exists: bool,
        name: CqlQualifiedIdentifier<I>,
        fields: Vec<(CqlIdentifier<I>, CqlType<UdtType>)>,
    ) -> Self {
        Self {
            if_not_exists,
            name,
            fields,
        }
    }

    /// Returns true if the user-defined type should only be created if it does not exist.
    pub fn if_not_exists(&self) -> bool {
        self.if_not_exists
    }

    /// Returns the name of the user-defined type.
    pub fn name(&self) -> &CqlQualifiedIdentifier<I> {
        &self.name
    }

    /// Returns the fields of the user-defined type.
    pub fn fields(&self) -> &Vec<(CqlIdentifier<I>, CqlType<UdtType>)> {
        &self.fields
    }
}