use crate::model::order::CqlOrder;

/// The cql table options.
/// More Information: <https://cassandra.apache.org/doc/latest/cassandra/cql/ddl.html#create-table-statement>
#[derive(Debug, Clone, PartialEq)]
pub struct CqlTableOptions<T, ColumnRef> {
    compact_storage: bool,
    /// The clustering order.
    clustering_order: Vec<(ColumnRef, CqlOrder)>,
    /// The other options.
    options: Vec<(T, T)>,
}

impl<T, ColumnRef> CqlTableOptions<T, ColumnRef> {
    /// Creates a new cql table options.
    pub fn new(compact_storage: bool, clustering_order: Vec<(ColumnRef, CqlOrder)>, options: Vec<(T, T)>) -> Self {
        Self {
            compact_storage,
            clustering_order,
            options,
        }
    }

    /// Returns true if the table has the compact storage option.
    pub fn has_compact_storage(&self) -> bool {
        self.compact_storage
    }

    /// Returns the clustering order.
    pub fn clustering_order(&self) -> &Vec<(ColumnRef, CqlOrder)> {
        &self.clustering_order
    }

    /// Returns the other options.
    pub fn options(&self) -> &Vec<(T, T)> {
        &self.options
    }
}