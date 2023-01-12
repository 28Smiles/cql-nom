use crate::model::cql_type::CqlType;
use crate::model::identifier::CqlIdentifier;
use derive_where::derive_where;

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
