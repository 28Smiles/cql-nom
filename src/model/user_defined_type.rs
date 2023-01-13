use crate::model::cql_type::CqlType;
use crate::model::identifier::CqlIdentifier;
use crate::model::qualified_identifier::CqlQualifiedIdentifier;
use crate::model::statement::CqlStatement;
use crate::model::Identifiable;
use derive_where::derive_where;
use std::ops::Deref;
use std::rc::Rc;

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
#[derive_where(PartialEq; UdtTypeRef, I: std::ops::Deref<Target = str> + std::cmp::PartialEq)]
pub struct ParsedCqlUserDefinedType<I, UdtTypeRef> {
    if_not_exists: bool,
    /// The name of the user-defined type.
    name: CqlQualifiedIdentifier<I>,
    /// The fields of the user-defined type.
    fields: Vec<(CqlIdentifier<I>, CqlType<UdtTypeRef>)>,
}

impl<I, UdtTypeRef> ParsedCqlUserDefinedType<I, UdtTypeRef> {
    /// Creates a new user-defined type.
    pub fn new(
        if_not_exists: bool,
        name: CqlQualifiedIdentifier<I>,
        fields: Vec<(CqlIdentifier<I>, CqlType<UdtTypeRef>)>,
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
    pub fn fields(&self) -> &Vec<(CqlIdentifier<I>, CqlType<UdtTypeRef>)> {
        &self.fields
    }
}

impl<I: Clone + Deref<Target = str>, UdtTypeRef> Identifiable<I>
    for ParsedCqlUserDefinedType<I, UdtTypeRef>
{
    fn keyspace(&self) -> Option<&CqlIdentifier<I>> {
        self.name.keyspace()
    }

    fn identifier(&self) -> &CqlIdentifier<I> {
        self.name.identifier()
    }
}

impl<I, UdtTypeRef> ParsedCqlUserDefinedType<I, UdtTypeRef> {
    pub(crate) fn reference_types<Table>(
        self,
        keyspace: Option<&CqlIdentifier<I>>,
        context: &Vec<CqlStatement<Table, Rc<CqlUserDefinedType<I>>>>,
    ) -> Result<CqlUserDefinedType<I>, CqlQualifiedIdentifier<I>>
    where
        I: Deref<Target = str> + Clone,
        UdtTypeRef: Identifiable<I>,
    {
        let keyspace = self.name.keyspace().or(keyspace);
        let fields = self
            .fields
            .into_iter()
            .map(|(name, cql_type)| {
                cql_type
                    .reference_types(keyspace, context)
                    .map(|cql_type| (name, cql_type))
            })
            .collect::<Result<Vec<_>, CqlQualifiedIdentifier<I>>>()?;
        Ok(CqlUserDefinedType::new(
            self.if_not_exists,
            self.name,
            fields,
        ))
    }
}

#[derive(Debug, Clone)]
#[derive_where(PartialEq; I: std::ops::Deref<Target = str> + std::cmp::PartialEq)]
pub struct CqlUserDefinedType<I> {
    if_not_exists: bool,
    /// The name of the user-defined type.
    name: CqlQualifiedIdentifier<I>,
    /// The fields of the user-defined type.
    fields: Vec<(CqlIdentifier<I>, CqlType<Rc<CqlUserDefinedType<I>>>)>,
}

impl<I> CqlUserDefinedType<I> {
    /// Creates a new user-defined type.
    pub fn new(
        if_not_exists: bool,
        name: CqlQualifiedIdentifier<I>,
        fields: Vec<(CqlIdentifier<I>, CqlType<Rc<CqlUserDefinedType<I>>>)>,
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
    pub fn fields(&self) -> &Vec<(CqlIdentifier<I>, CqlType<Rc<CqlUserDefinedType<I>>>)> {
        &self.fields
    }
}

impl<I: Clone + Deref<Target = str>> Identifiable<I> for CqlUserDefinedType<I> {
    fn keyspace(&self) -> Option<&CqlIdentifier<I>> {
        self.name.keyspace()
    }

    fn identifier(&self) -> &CqlIdentifier<I> {
        self.name.identifier()
    }
}
