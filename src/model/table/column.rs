use crate::model::*;
use derive_new::new;
use derive_where::derive_where;
use getset::{CopyGetters, Getters};
use std::ops::Deref;
use std::rc::Rc;

/// The cql column.
/// More Information: <https://cassandra.apache.org/doc/latest/cassandra/cql/ddl.html#create-table-statement>
#[derive(Debug, Clone, Getters, CopyGetters, new)]
#[derive_where(PartialEq; UdtType, I: std::ops::Deref<Target = str>)]
pub struct CqlColumn<I, UdtType> {
    /// The name of the column.
    #[getset(get = "pub")]
    name: CqlIdentifier<I>,
    /// The type of the column.
    #[getset(get = "pub")]
    cql_type: CqlType<UdtType>,
    /// Whether the column is static.
    #[getset(get_copy = "pub")]
    is_static: bool,
    /// Whether the column is part of the primary key.
    #[getset(get_copy = "pub")]
    is_primary_key: bool,
}

impl<I: Clone, UdtType> Identifiable<I> for CqlColumn<I, UdtType> {
    fn keyspace(&self) -> Option<&CqlIdentifier<I>> {
        None
    }

    fn identifier(&self) -> &CqlIdentifier<I> {
        self.name()
    }
}

impl<I, UdtTypeRef> CqlColumn<I, UdtTypeRef> {
    pub(crate) fn reference_types<Table, UdtType>(
        self,
        keyspace: Option<&CqlIdentifier<I>>,
        context: &Vec<CqlStatement<Table, Rc<UdtType>>>,
    ) -> Result<CqlColumn<I, Rc<UdtType>>, CqlQualifiedIdentifier<I>>
    where
        I: Deref<Target = str> + Clone,
        UdtTypeRef: Identifiable<I>,
        UdtType: Identifiable<I>,
    {
        Ok(CqlColumn::new(
            self.name,
            self.cql_type.reference_types(keyspace, context)?,
            self.is_static,
            self.is_primary_key,
        ))
    }
}
