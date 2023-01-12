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

use nom::bytes::complete::tag;
use nom::IResult;
use nom::multi::separated_list0;
use crate::model::identifier::CqlIdentifier;
use crate::model::statement::CqlStatement;
use crate::parse::Parse;
use crate::utils::space0_around;

pub mod error;
pub mod model;
mod parse;
mod utils;
mod ast;

pub fn parse_cql(input: &str) -> IResult<&str, Vec<CqlStatement<&str, CqlIdentifier<&str>, CqlIdentifier<&str>>>> {
    space0_around(separated_list0(tag(";"), space0_around(CqlStatement::parse)))(input)
}
