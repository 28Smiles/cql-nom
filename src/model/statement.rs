#[derive(Debug, Clone, PartialEq)]
pub enum CqlStatement<Table, UdtType> {
    /// A `CREATE TABLE` statement.
    CreateTable(Table),
    /// A `CREATE TYPE` statement.
    CreateUserDefinedType(UdtType),
}

impl<Table, UdtType> CqlStatement<Table, UdtType> {
    /// Returns `true` if the statement is a `CREATE TABLE` statement.
    pub fn is_create_table(&self) -> bool {
        match *self {
            CqlStatement::CreateTable(_) => true,
            _ => false,
        }
    }

    /// Returns `true` if the statement is a `CREATE TYPE` statement.
    pub fn is_create_user_defined_type(&self) -> bool {
        match *self {
            CqlStatement::CreateUserDefinedType(_) => true,
            _ => false,
        }
    }

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
