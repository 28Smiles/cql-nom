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
use nom::character::complete::multispace0;
use nom::combinator::opt;
use nom::multi::separated_list0;
use nom::IResult;
use std::rc::Rc;

/// The tree elements of the Cassandra Query Language.
pub mod model;
mod parse;
mod utils;

/// Parses a CQL statement into a tree.
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
    let (input, statements) = separated_list0(tag(";"), space0_around(CqlStatement::parse))(input)?;
    let (input, _) = opt(tag(";"))(input)?;
    let (input, _) = multispace0(input)?;

    Ok((input, statements))
}

/// Resolves the identifiers of the CQL statements.
pub fn resolve_references<'a>(
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::model::*;

    #[test]
    fn test() {
        let input = r#"
        CREATE TYPE IF NOT EXISTS my_keyspace.my_type (
            my_field1 int,
            my_field2 text,
            my_field3 frozen<list<text>>,
            my_field4 frozen<map<text, text>>,
            my_field5 frozen<set<text>>,
            my_field6 frozen<tuple<text, int>>,
            my_field7 inet,
            my_field8 timestamp,
            my_field9 timeuuid,
            my_field10 uuid,
            my_field11 bigint,
            my_field12 blob,
            my_field13 boolean,
            my_field14 date,
            my_field15 decimal,
            my_field16 double,
            my_field17 float
        );

        CREATE TYPE my_keyspace."my_type2" (
            my_field1 int,
            my_field2 frozen<my_type>
        );

        CREATE TABLE my_keyspace."my_table" (
            my_field1 int,
            my_field2 text,
            my_field3 frozen<my_type2>,

            PRIMARY KEY (my_field1, my_field2)
        ) WITH CLUSTERING ORDER BY (my_field2 DESC);
        "#;

        let (remaining, parse_tree) = super::parse_cql(input).unwrap();
        assert_eq!(remaining, "");
        let ast = super::resolve_references(parse_tree, None).unwrap();
        let my_type = ast[0].create_user_defined_type().unwrap();
        let my_type2 = ast[1].create_user_defined_type().unwrap();
        let my_table = ast[2].create_table().unwrap();
        let my_type_ref = Rc::new(CqlUserDefinedType::new(
            true,
            CqlQualifiedIdentifier::new(
                Some(CqlIdentifier::new("my_keyspace")),
                CqlIdentifier::new("my_type"),
            ),
            vec![
                (CqlIdentifier::new("my_field1"), CqlType::INT),
                (CqlIdentifier::new("my_field2"), CqlType::TEXT),
                (
                    CqlIdentifier::new("my_field3"),
                    CqlType::FROZEN(Box::new(CqlType::LIST(Box::new(CqlType::TEXT)))),
                ),
                (
                    CqlIdentifier::new("my_field4"),
                    CqlType::FROZEN(Box::new(CqlType::MAP(Box::new((
                        CqlType::TEXT,
                        CqlType::TEXT,
                    ))))),
                ),
                (
                    CqlIdentifier::new("my_field5"),
                    CqlType::FROZEN(Box::new(CqlType::SET(Box::new(CqlType::TEXT)))),
                ),
                (
                    CqlIdentifier::new("my_field6"),
                    CqlType::FROZEN(Box::new(CqlType::TUPLE(vec![CqlType::TEXT, CqlType::INT]))),
                ),
                (CqlIdentifier::new("my_field7"), CqlType::INET),
                (CqlIdentifier::new("my_field8"), CqlType::TIMESTAMP),
                (CqlIdentifier::new("my_field9"), CqlType::TIMEUUID),
                (CqlIdentifier::new("my_field10"), CqlType::UUID),
                (CqlIdentifier::new("my_field11"), CqlType::BIGINT),
                (CqlIdentifier::new("my_field12"), CqlType::BLOB),
                (CqlIdentifier::new("my_field13"), CqlType::BOOLEAN),
                (CqlIdentifier::new("my_field14"), CqlType::DATE),
                (CqlIdentifier::new("my_field15"), CqlType::DECIMAL),
                (CqlIdentifier::new("my_field16"), CqlType::DOUBLE),
                (CqlIdentifier::new("my_field17"), CqlType::FLOAT),
            ],
        ));
        assert_eq!(my_type, &my_type_ref);
        let my_type2_ref = Rc::new(CqlUserDefinedType::new(
            false,
            CqlQualifiedIdentifier::new(
                Some(CqlIdentifier::new("my_keyspace")),
                CqlIdentifier::new("my_type2"),
            ),
            vec![
                (CqlIdentifier::new("my_field1"), CqlType::INT),
                (
                    CqlIdentifier::new("my_field2"),
                    CqlType::FROZEN(Box::new(CqlType::UserDefined(my_type_ref.clone()))),
                ),
            ],
        ));
        assert_eq!(my_type2, &my_type2_ref);
        let column_my_field1 = Rc::new(CqlColumn::new(
            CqlIdentifier::Unquoted("my_field1"),
            CqlType::INT,
            false,
            false,
        ));
        let column_my_field2 = Rc::new(CqlColumn::new(
            CqlIdentifier::Unquoted("my_field2"),
            CqlType::TEXT,
            false,
            false,
        ));
        let column_my_field3 = Rc::new(CqlColumn::new(
            CqlIdentifier::Unquoted("my_field3"),
            CqlType::FROZEN(Box::new(CqlType::UserDefined(my_type2_ref.clone()))),
            false,
            false,
        ));
        let my_table_ref = Rc::new(CqlTable::new(
            false,
            CqlQualifiedIdentifier::new(
                Some(CqlIdentifier::Unquoted("my_keyspace")),
                CqlIdentifier::Quoted("my_table".to_string()),
            ),
            vec![
                column_my_field1.clone(),
                column_my_field2.clone(),
                column_my_field3.clone(),
            ],
            Some(CqlPrimaryKey::new(
                vec![column_my_field1.clone()],
                vec![column_my_field2.clone()],
            )),
            Some(CqlTableOptions::new(
                false,
                vec![(column_my_field2.clone(), CqlOrder::Desc)],
                vec![],
            )),
        ));
        assert_eq!(my_table, &my_table_ref);
    }
}
