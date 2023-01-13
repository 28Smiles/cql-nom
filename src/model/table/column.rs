use std::ops::Deref;
use std::rc::Rc;
use crate::model::cql_type::CqlType;
use crate::model::identifier::CqlIdentifier;
use crate::model::qualified_identifier::CqlQualifiedIdentifier;
use crate::model::Identifiable;
use derive_where::derive_where;
use crate::model::statement::CqlStatement;

/// The cql column.
/// More Information: <https://cassandra.apache.org/doc/latest/cassandra/cql/ddl.html#create-table-statement>
#[derive(Debug, Clone)]
#[derive_where(PartialEq; UdtType, I: std::ops::Deref<Target = str>)]
pub struct CqlColumn<I, UdtType> {
    /// The name of the column.
    name: CqlIdentifier<I>,
    /// The type of the column.
    cql_type: CqlType<UdtType>,
    /// Whether the column is static.
    is_static: bool,
    /// Whether the column is part of the primary key.
    is_primary_key: bool,
}

impl<I, UdtType> CqlColumn<I, UdtType> {
    /// Creates a new cql column.
    pub fn new(
        name: CqlIdentifier<I>,
        cql_type: CqlType<UdtType>,
        is_static: bool,
        is_primary_key: bool,
    ) -> Self {
        Self {
            name,
            cql_type,
            is_static,
            is_primary_key,
        }
    }

    /// Returns the name of the column.
    pub fn name(&self) -> &CqlIdentifier<I> {
        &self.name
    }

    /// Returns the type of the column.
    pub fn cql_type(&self) -> &CqlType<UdtType> {
        &self.cql_type
    }

    /// Returns whether the column is static.
    pub fn is_static(&self) -> bool {
        self.is_static
    }

    /// Returns whether the column is part of the primary key.
    pub fn is_primary_key(&self) -> bool {
        self.is_primary_key
    }
}

impl<I: Clone, UdtType> Identifiable<I> for CqlColumn<I, UdtType> {
    fn keyspace(&self) -> Option<&CqlIdentifier<I>> {
        None
    }

    fn identifier(&self) -> &CqlIdentifier<I> {
        &self.name
    }
}

impl<I, UdtTypeRef> CqlColumn<I, UdtTypeRef> {
    pub(crate) fn reference_types<Table, UdtType>(
        self,
        keyspace: Option<&CqlIdentifier<I>>,
        context: &Vec<CqlStatement<Table, Rc<UdtType>>>,
    ) -> Result<CqlColumn<I, Rc<UdtType>>, CqlQualifiedIdentifier<I>>
        where
            I: Deref<Target = str> + Clone,
            UdtTypeRef: Identifiable<I>,
            UdtType: Identifiable<I>,
    {
        Ok(CqlColumn::new(
            self.name,
            self.cql_type.reference_types(keyspace, context)?,
            self.is_static,
            self.is_primary_key,
        ))
    }
}

