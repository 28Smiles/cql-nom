use crate::model::*;
use derive_new::new;
use derive_where::derive_where;
use getset::{CopyGetters, Getters};
use std::ops::Deref;
use std::rc::Rc;

/// A column of a table.
pub mod column;
/// The table options.
pub mod options;
/// The table primary key definition.
pub mod primary_key;

pub use column::*;
pub use options::*;
pub use primary_key::*;

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
#[derive(Debug, Clone, Getters, CopyGetters, new)]
#[derive_where(PartialEq; Column, ColumnRef, I: std::ops::Deref<Target = str> + std::cmp::PartialEq)]
pub struct CqlTable<I, Column, ColumnRef> {
    /// If the table should only be created if it does not exist.
    #[getset(get_copy = "pub")]
    if_not_exists: bool,
    /// The name of the table.
    #[getset(get = "pub")]
    name: CqlQualifiedIdentifier<I>,
    /// The columns of the table.
    #[getset(get = "pub")]
    columns: Vec<Column>,
    /// The primary key of the table.
    #[getset(get = "pub")]
    primary_key: Option<CqlPrimaryKey<ColumnRef>>,
    /// The table options.
    #[getset(get = "pub")]
    options: Option<CqlTableOptions<I, ColumnRef>>,
}

impl<I: Clone + Deref<Target = str>, Column, ColumnRef> Identifiable<I>
    for CqlTable<I, Column, ColumnRef>
{
    #[inline(always)]
    fn keyspace(&self) -> Option<&CqlIdentifier<I>> {
        self.name.keyspace().as_ref()
    }
    #[inline(always)]
    fn identifier(&self) -> &CqlIdentifier<I> {
        self.name.identifier()
    }
}

impl<I, UdtTypeRef, ColumnRef> CqlTable<I, CqlColumn<I, UdtTypeRef>, ColumnRef> {
    pub(crate) fn reference_types<Table>(
        self,
        keyspace: Option<&CqlIdentifier<I>>,
        context: &Vec<CqlStatement<Table, Rc<CqlUserDefinedType<I>>>>,
    ) -> Result<
        CqlTable<
            I,
            Rc<CqlColumn<I, Rc<CqlUserDefinedType<I>>>>,
            Rc<CqlColumn<I, Rc<CqlUserDefinedType<I>>>>,
        >,
        CqlQualifiedIdentifier<I>,
    >
    where
        I: Deref<Target = str> + Clone,
        ColumnRef: Identifiable<I>,
        UdtTypeRef: Identifiable<I>,
    {
        let keyspace = self.name.contextualized_keyspace(keyspace);
        let columns = self
            .columns
            .into_iter()
            .map(|column| {
                column
                    .reference_types(keyspace.as_ref(), context)
                    .map(Rc::new)
            })
            .collect::<Result<Vec<_>, _>>()?;
        let primary_key = self
            .primary_key
            .map(|primary_key| primary_key.reference_types(keyspace.as_ref(), &columns))
            .transpose()?;
        let options = self
            .options
            .map(|options| options.reference_types(keyspace.as_ref(), &columns))
            .transpose()?;

        Ok(CqlTable::new(
            self.if_not_exists,
            self.name,
            columns,
            primary_key,
            options,
        ))
    }
}
