use std::ops::Deref;
use std::rc::Rc;
use crate::model::Identifiable;
use crate::model::identifier::CqlIdentifier;
use crate::model::order::CqlOrder;
use crate::model::qualified_identifier::CqlQualifiedIdentifier;
use crate::model::table::column::CqlColumn;

/// The cql table options.
/// More Information: <https://cassandra.apache.org/doc/latest/cassandra/cql/ddl.html#create-table-statement>
#[derive(Debug, Clone, PartialEq)]
pub struct CqlTableOptions<I, ColumnRef> {
    compact_storage: bool,
    /// The clustering order.
    clustering_order: Vec<(ColumnRef, CqlOrder)>,
    /// The other options.
    options: Vec<(I, I)>,
}

impl<I, ColumnRef> CqlTableOptions<I, ColumnRef> {
    /// Creates a new cql table options.
    pub fn new(
        compact_storage: bool,
        clustering_order: Vec<(ColumnRef, CqlOrder)>,
        options: Vec<(I, I)>,
    ) -> Self {
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
    pub fn options(&self) -> &Vec<(I, I)> {
        &self.options
    }
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
        let clustering_order = self.clustering_order.into_iter()
            .map(|(column, order)| {
                table_context.iter()
                    .find(|c| c.contextualized_identifier(keyspace) == column.contextualized_identifier(keyspace))
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
