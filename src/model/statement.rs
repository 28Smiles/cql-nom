use crate::model::table::CqlTable;
use crate::model::user_defined_type::CqlUserDefinedType;
use derive_where::derive_where;
use crate::model::qualified_identifier::CqlQualifiedIdentifier;

#[derive(Debug, Clone)]
#[derive_where(PartialEq; UdtType, ColumnRef, I: std::ops::Deref<Target = str> + std::cmp::PartialEq)]
pub enum CqlStatement<I, UdtType, ColumnRef> {
    /// A `CREATE TABLE` statement.
    CreateTable(CqlTable<I, UdtType, ColumnRef>),
    /// A `CREATE TYPE` statement.
    CreateUserDefinedType(CqlUserDefinedType<I, UdtType>),
}

impl<I, UdtType, ColumnRef> CqlStatement<I, UdtType, ColumnRef> {
    /// Returns true if the statement is a `CREATE TABLE` statement.
    pub fn is_create_table(&self) -> bool {
        match self {
            Self::CreateTable(_) => true,
            _ => false,
        }
    }

    /// Returns true if the statement is a `CREATE TYPE` statement.
    pub fn is_create_user_defined_type(&self) -> bool {
        match self {
            Self::CreateUserDefinedType(_) => true,
            _ => false,
        }
    }

    /// Returns the `CREATE TABLE` statement if the statement is a `CREATE TABLE` statement.
    pub fn create_table(&self) -> Option<&CqlTable<I, UdtType, ColumnRef>> {
        match self {
            Self::CreateTable(create_table) => Some(create_table),
            _ => None,
        }
    }

    /// Returns the `CREATE TYPE` statement if the statement is a `CREATE TYPE` statement.
    pub fn create_user_defined_type(&self) -> Option<&CqlUserDefinedType<I, UdtType>> {
        match self {
            Self::CreateUserDefinedType(create_user_defined_type) => Some(create_user_defined_type),
            _ => None,
        }
    }
}

impl<I, UdtType, ColumnRef> CqlStatement<I, UdtType, ColumnRef> where I: std::ops::Deref<Target = str> + std::cmp::PartialEq {
    /// Returns the name of the statement.
    pub fn name(&self) -> Option<&CqlQualifiedIdentifier<I>> {
        match self {
            Self::CreateTable(create_table) => Some(create_table.name()),
            Self::CreateUserDefinedType(create_user_defined_type) => Some(create_user_defined_type.name()),
        }
    }
}