use crate::model::identifier::CqlIdentifier;
use crate::model::order::CqlOrder;
use crate::model::qualified_identifier::CqlQualifiedIdentifier;
use crate::model::table::column::CqlColumn;
use crate::model::Identifiable;
use derive_new::new;
use getset::{CopyGetters, Getters};
use std::ops::Deref;
use std::rc::Rc;

/// The cql table options.
/// More Information: <https://cassandra.apache.org/doc/latest/cassandra/cql/ddl.html#create-table-statement>
#[derive(Debug, Clone, PartialEq, Getters, CopyGetters, new)]
pub struct CqlTableOptions<I, ColumnRef> {
    /// Has the compact storage keyword.
    #[getset(get_copy = "pub")]
    compact_storage: bool,
    /// The clustering order.
    #[getset(get = "pub")]
    clustering_order: Vec<(ColumnRef, CqlOrder)>,
    /// The other options.
    #[getset(get = "pub")]
    options: Vec<(I, I)>,
}

impl<I, ColumnRef> CqlTableOptions<I, ColumnRef> {
    pub(crate) fn reference_types<UdtType>(
        self,
        keyspace: Option<&CqlIdentifier<I>>,
        table_context: &Vec<Rc<CqlColumn<I, Rc<UdtType>>>>,
    ) -> Result<CqlTableOptions<I, Rc<CqlColumn<I, Rc<UdtType>>>>, CqlQualifiedIdentifier<I>>
    where
        I: Deref<Target = str> + Clone,
        ColumnRef: Identifiable<I>,
    {
        let clustering_order = self
            .clustering_order
            .into_iter()
            .map(|(column, order)| {
                table_context
                    .iter()
                    .find(|c| {
                        c.contextualized_identifier(keyspace)
                            == column.contextualized_identifier(keyspace)
                    })
                    .map(|column| (Rc::clone(column), order))
                    .ok_or_else(|| column.contextualized_identifier(keyspace))
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(CqlTableOptions::new(
            self.compact_storage,
            clustering_order,
            self.options,
        ))
    }
}
