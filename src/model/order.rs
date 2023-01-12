/// The cql order.
/// More Information: <https://cassandra.apache.org/doc/latest/cassandra/cql/ddl.html#create-table-statement>
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CqlOrder {
    Asc,
    Desc,
}
