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
