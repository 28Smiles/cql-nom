use crate::model::*;
use derive_new::new;
use getset::Getters;
use std::ops::Deref;
use std::rc::Rc;

/// The cql primary key.
/// More Information: <https://cassandra.apache.org/doc/latest/cassandra/cql/ddl.html#create-table-statement>
#[derive(Debug, Clone, PartialEq, Getters, new)]
pub struct CqlPrimaryKey<ColumnRef> {
    /// The partition key.
    #[getset(get = "pub")]
    partition_key: Vec<ColumnRef>,
    /// The clustering columns.
    #[getset(get = "pub")]
    clustering_columns: Vec<ColumnRef>,
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
        let partition_key = self
            .partition_key
            .into_iter()
            .map(|column| {
                table_context
                    .iter()
                    .find(|c| {
                        c.contextualized_identifier(keyspace)
                            == column.contextualized_identifier(keyspace)
                    })
                    .ok_or_else(|| column.contextualized_identifier(keyspace))
                    .map(Rc::clone)
            })
            .collect::<Result<Vec<_>, _>>()?;
        let clustering_columns = self
            .clustering_columns
            .into_iter()
            .map(|column| {
                table_context
                    .iter()
                    .find(|c| {
                        c.contextualized_identifier(keyspace)
                            == column.contextualized_identifier(keyspace)
                    })
                    .ok_or_else(|| column.contextualized_identifier(keyspace))
                    .map(Rc::clone)
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(CqlPrimaryKey::new(partition_key, clustering_columns))
    }
}
