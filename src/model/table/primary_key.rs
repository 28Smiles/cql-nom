use std::ops::Deref;
use std::rc::Rc;
use crate::model::Identifiable;
use crate::model::identifier::CqlIdentifier;
use crate::model::qualified_identifier::CqlQualifiedIdentifier;
use crate::model::table::column::CqlColumn;

/// The cql primary key.
/// More Information: <https://cassandra.apache.org/doc/latest/cassandra/cql/ddl.html#create-table-statement>
#[derive(Debug, Clone, PartialEq)]
pub struct CqlPrimaryKey<ColumnRef> {
    /// The partition key.
    partition_key: Vec<ColumnRef>,
    /// The clustering columns.
    clustering_columns: Vec<ColumnRef>,
}

impl<ColumnRef> CqlPrimaryKey<ColumnRef> {
    /// Creates a new primary key.
    pub fn new(partition_key: Vec<ColumnRef>, clustering_columns: Vec<ColumnRef>) -> Self {
        Self {
            partition_key,
            clustering_columns,
        }
    }

    /// Returns the partition key.
    pub fn partition_key(&self) -> &Vec<ColumnRef> {
        &self.partition_key
    }

    /// Returns the clustering columns.
    pub fn clustering_columns(&self) -> &Vec<ColumnRef> {
        &self.clustering_columns
    }
}

impl<ColumnRef> CqlPrimaryKey<ColumnRef> {
    pub(crate) fn reference_types<I, UdtType>(
        self,
        keyspace: Option<&CqlIdentifier<I>>,
        table_context: &Vec<Rc<CqlColumn<I, Rc<UdtType>>>>,
    ) -> Result<CqlPrimaryKey<Rc<CqlColumn<I, Rc<UdtType>>>>, CqlQualifiedIdentifier<I>>
        where
            I: Deref<Target = str> + Clone,
            ColumnRef: Identifiable<I>,
    {
        let partition_key = self.partition_key.into_iter()
            .map(|column| {
                table_context.iter()
                    .find(|c| c.contextualized_identifier(keyspace) == column.contextualized_identifier(keyspace))
                    .ok_or_else(|| column.contextualized_identifier(keyspace))
                    .map(Rc::clone)
            })
            .collect::<Result<Vec<_>, _>>()?;
        let clustering_columns = self.clustering_columns.into_iter()
            .map(|column| {
                table_context.iter()
                    .find(|c| c.contextualized_identifier(keyspace) == column.contextualized_identifier(keyspace))
                    .ok_or_else(|| column.contextualized_identifier(keyspace))
                    .map(Rc::clone)
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(CqlPrimaryKey::new(partition_key, clustering_columns))
    }
}