use derive_more::IsVariant;

/// The cql order.
/// More Information: <https://cassandra.apache.org/doc/latest/cassandra/cql/ddl.html#create-table-statement>
#[derive(Debug, Copy, Clone, PartialEq, IsVariant)]
pub enum CqlOrder {
    /// Ascending order.
    Asc,
    /// Descending order.
    Desc,
}
