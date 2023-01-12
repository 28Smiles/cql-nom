pub mod column;
pub mod options;
pub mod primary_key;

use crate::model::identifier::CqlIdentifier;
use crate::model::qualified_identifier::CqlQualifiedIdentifier;
use crate::model::statement::CqlStatement;
use crate::model::table::column::CqlColumn;
use crate::model::table::options::CqlTableOptions;
use crate::model::table::primary_key::CqlPrimaryKey;
use crate::model::Identifiable;
use derive_where::derive_where;
use std::ops::Deref;
use std::rc::Rc;

/// The cql table.
/// More Information: <https://cassandra.apache.org/doc/latest/cassandra/cql/ddl.html#create-table-statement>
///
/// Grammar:
/// ```bnf
/// create_table_statement::= CREATE TABLE [ IF NOT EXISTS ] table_name '('
/// 	column_definition  ( ',' column_definition )*
/// 	[ ',' PRIMARY KEY '(' primary_key ')' ]
/// 	 ')' [ WITH table_options ]
/// column_definition::= column_name cql_type [ STATIC ] [ PRIMARY KEY]
/// primary_key::= partition_key [ ',' clustering_columns ]
/// partition_key::= column_name  | '(' column_name ( ',' column_name )* ')'
/// clustering_columns::= column_name ( ',' column_name )*
/// table_options:=: COMPACT STORAGE [ AND table_options ]
/// 	| CLUSTERING ORDER BY '(' clustering_order ')'
/// 	[ AND table_options ]  | options
/// clustering_order::= column_name (ASC | DESC) ( ',' column_name (ASC | DESC) )*
/// ```
///
/// Example:
/// ```cql
/// CREATE TABLE monkey_species (
///     species text PRIMARY KEY,
///     common_name text,
///     population varint,
///     average_size int
/// ) WITH comment='Important biological records';
///
/// CREATE TABLE timeline (
///     userid uuid,
///     posted_month int,
///     posted_time uuid,
///     body text,
///     posted_by text,
///     PRIMARY KEY (userid, posted_month, posted_time)
/// ) WITH compaction = { 'class' : 'LeveledCompactionStrategy' };
///
/// CREATE TABLE loads (
///     machine inet,
///     cpu int,
///     mtime timeuuid,
///     load float,
///     PRIMARY KEY ((machine, cpu), mtime)
/// ) WITH CLUSTERING ORDER BY (mtime DESC);
/// ```
#[derive(Debug, Clone)]
#[derive_where(PartialEq; Column, ColumnRef, I: std::ops::Deref<Target = str> + std::cmp::PartialEq)]
pub struct CqlTable<I, Column, ColumnRef> {
    /// If the table should only be created if it does not exist.
    if_not_exists: bool,
    /// The name of the table.
    name: CqlQualifiedIdentifier<I>,
    /// The columns of the table.
    columns: Vec<Column>,
    /// The primary key of the table.
    primary_key: Option<CqlPrimaryKey<ColumnRef>>,
    /// The table options.
    options: Option<CqlTableOptions<I, ColumnRef>>,
}

impl<I, Column, ColumnRef> CqlTable<I, Column, ColumnRef> {
    /// Creates a new table.
    pub fn new(
        if_not_exists: bool,
        name: CqlQualifiedIdentifier<I>,
        columns: Vec<Column>,
        primary_key: Option<CqlPrimaryKey<ColumnRef>>,
        options: Option<CqlTableOptions<I, ColumnRef>>,
    ) -> Self {
        Self {
            if_not_exists,
            name,
            columns,
            primary_key,
            options,
        }
    }

    /// Returns true if the table should only be created if it does not exist.
    pub fn if_not_exists(&self) -> bool {
        self.if_not_exists
    }

    /// Returns the name of the table.
    pub fn name(&self) -> &CqlQualifiedIdentifier<I> {
        &self.name
    }

    /// Returns the columns of the table.
    pub fn columns(&self) -> &Vec<Column> {
        &self.columns
    }

    /// Returns the primary key of the table.
    pub fn primary_key(&self) -> Option<&CqlPrimaryKey<ColumnRef>> {
        self.primary_key.as_ref()
    }

    /// Returns the table options.
    pub fn options(&self) -> Option<&CqlTableOptions<I, ColumnRef>> {
        self.options.as_ref()
    }

    /// Wraps referenceable values in an Rc.
    pub(crate) fn with_rc(self) -> CqlTable<I, Rc<Column>, ColumnRef> {
        CqlTable {
            if_not_exists: self.if_not_exists,
            name: self.name,
            columns: self.columns.into_iter().map(Rc::new).collect(),
            primary_key: self.primary_key,
            options: self.options,
        }
    }
}

impl<I: Clone + Deref<Target = str>, Column, ColumnRef> Identifiable<I>
    for CqlTable<I, Column, ColumnRef>
{
    fn identifier(&self, keyspace: Option<&CqlIdentifier<I>>) -> CqlQualifiedIdentifier<I> {
        self.name.identifier(keyspace)
    }
}
