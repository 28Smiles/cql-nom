//! # Cql-Nom, a parser for the Cassandra Query Language
//!
//! This crate provides a parser for the Cassandra Query Language (CQL).
//! It is based on the [nom](https://github.com/rust-bakery/nom) parser combinator library.
//!
//! ## Example
//! ```rust
//! use cql_nom::parse_cql;
//!
//! let input = r#"CREATE TABLE IF NOT EXISTS my_keyspace.my_table (
//!     my_field1 int,
//!     my_field2 text,
//!     PRIMARY KEY (my_field1)
//! ) WITH CLUSTERING ORDER BY (my_field2 DESC);"#;
//!
//! let result = parse_cql(input).unwrap();
//! ```
//!
//! The code is available on [GitHub](https://github.com/28Smiles/cql-nom).

use crate::model::identifier::CqlIdentifier;
use crate::model::qualified_identifier::CqlQualifiedIdentifier;
use crate::model::statement::CqlStatement;
use crate::model::table::column::CqlColumn;
use crate::model::table::CqlTable;
use crate::model::user_defined_type::{CqlUserDefinedType, ParsedCqlUserDefinedType};
use crate::parse::Parse;
use crate::utils::space0_around;
use nom::bytes::complete::tag;
use nom::multi::separated_list0;
use nom::IResult;
use std::rc::Rc;

pub mod error;
pub mod model;
mod parse;
mod utils;

pub fn parse_cql(
    input: &str,
) -> IResult<
    &str,
    Vec<
        CqlStatement<
            CqlTable<&str, CqlColumn<&str, CqlIdentifier<&str>>, CqlIdentifier<&str>>,
            ParsedCqlUserDefinedType<&str, CqlIdentifier<&str>>,
        >,
    >,
> {
    space0_around(separated_list0(
        tag(";"),
        space0_around(CqlStatement::parse),
    ))(input)
}

pub fn parse_tree_to_ast<'a>(
    input: Vec<
        CqlStatement<
            CqlTable<&'a str, CqlColumn<&'a str, CqlIdentifier<&'a str>>, CqlIdentifier<&'a str>>,
            ParsedCqlUserDefinedType<&'a str, CqlIdentifier<&'a str>>,
        >,
    >,
    keyspace: Option<&'a CqlIdentifier<&'a str>>,
) -> Result<
    Vec<
        CqlStatement<
            Rc<
                CqlTable<
                    &'a str,
                    Rc<CqlColumn<&'a str, Rc<CqlUserDefinedType<&'a str>>>>,
                    Rc<CqlColumn<&'a str, Rc<CqlUserDefinedType<&'a str>>>>,
                >,
            >,
            Rc<CqlUserDefinedType<&'a str>>,
        >,
    >,
    CqlQualifiedIdentifier<&'a str>,
> {
    let mut result = Vec::new();
    for i in input {
        let i = i.reference_types(keyspace.clone(), &result)?;
        result.push(i);
    }

    Ok(result)
}
