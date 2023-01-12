use crate::model::identifier::CqlIdentifier;
use derive_where::derive_where;

/// The cql primary key.
/// More Information: <https://cassandra.apache.org/doc/latest/cassandra/cql/ddl.html#create-table-statement>
#[derive(Debug, Clone)]
#[derive_where(PartialEq; I: std::ops::Deref<Target = str>)]
pub struct CqlPrimaryKey<I> {
    /// The partition key.
    partition_key: Vec<CqlIdentifier<I>>,
    /// The clustering columns.
    clustering_columns: Vec<CqlIdentifier<I>>,
}

impl<I> CqlPrimaryKey<I> {
    /// Creates a new primary key.
    pub fn new(partition_key: Vec<CqlIdentifier<I>>, clustering_columns: Vec<CqlIdentifier<I>>) -> Self {
        Self {
            partition_key,
            clustering_columns,
        }
    }

    /// Returns the partition key.
    pub fn partition_key(&self) -> &Vec<CqlIdentifier<I>> {
        &self.partition_key
    }

    /// Returns the clustering columns.
    pub fn clustering_columns(&self) -> &Vec<CqlIdentifier<I>> {
        &self.clustering_columns
    }
}
