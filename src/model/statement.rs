use std::ops::Deref;
use std::rc::Rc;
use derive_more::IsVariant;
use crate::model::Identifiable;
use crate::model::identifier::CqlIdentifier;
use crate::model::qualified_identifier::CqlQualifiedIdentifier;
use crate::model::table::column::CqlColumn;
use crate::model::table::CqlTable;
use crate::model::user_defined_type::{CqlUserDefinedType, ParsedCqlUserDefinedType};

#[derive(Debug, Clone, PartialEq, IsVariant)]
pub enum CqlStatement<Table, UdtType> {
    /// A `CREATE TABLE` statement.
    CreateTable(Table),
    /// A `CREATE TYPE` statement.
    CreateUserDefinedType(UdtType),
}

impl<Table, UdtType> CqlStatement<Table, UdtType> {
    /// Returns the `CREATE TABLE` statement.
    pub fn create_table(&self) -> Option<&Table> {
        match *self {
            CqlStatement::CreateTable(ref table) => Some(table),
            _ => None,
        }
    }

    /// Returns the `CREATE TYPE` statement.
    pub fn create_user_defined_type(&self) -> Option<&UdtType> {
        match *self {
            CqlStatement::CreateUserDefinedType(ref udt_type) => Some(udt_type),
            _ => None,
        }
    }
}

impl<I, ColumnRef, UdtTypeRef> CqlStatement<CqlTable<I, CqlColumn<I, UdtTypeRef>, ColumnRef>, ParsedCqlUserDefinedType<I, UdtTypeRef>> {
    pub(crate) fn reference_types(
        self,
        keyspace: Option<&CqlIdentifier<I>>,
        context: &Vec<CqlStatement<
            Rc<CqlTable<I, Rc<CqlColumn<I, Rc<CqlUserDefinedType<I>>>>, Rc<CqlColumn<I, Rc<CqlUserDefinedType<I>>>>>>,
            Rc<CqlUserDefinedType<I>>,
        >>,
    ) -> Result<
            CqlStatement<
                Rc<CqlTable<
                    I,
                    Rc<CqlColumn<I, Rc<CqlUserDefinedType<I>>>>,
                    Rc<CqlColumn<I, Rc<CqlUserDefinedType<I>>>>
                >>,
                Rc<CqlUserDefinedType<I>>,
            >,
            CqlQualifiedIdentifier<I>,
        >
        where
            I: Deref<Target = str> + Clone,
            ColumnRef: Identifiable<I>,
            UdtTypeRef: Identifiable<I>,
    {
        match self {
            CqlStatement::CreateTable(table) => {
                Ok(CqlStatement::CreateTable(Rc::new(table.reference_types(keyspace, context)?)))
            }
            CqlStatement::CreateUserDefinedType(udt_type) => {
                Ok(CqlStatement::CreateUserDefinedType(Rc::new(udt_type.reference_types(keyspace, context)?)))
            }
        }
    }
}
