use nom::branch::alt;
use nom::combinator::map;
use nom::error::ParseError;
use nom::IResult;

use crate::model::identifier::CqlIdentifier;
use crate::model::statement::CqlStatement;
use crate::model::table::column::CqlColumn;
use crate::model::table::CqlTable;
use crate::model::user_defined_type::ParsedCqlUserDefinedType;
use crate::parse::Parse;

impl<'de, E: ParseError<&'de str>> Parse<&'de str, E>
    for CqlStatement<
        CqlTable<&'de str, CqlColumn<&'de str, CqlIdentifier<&'de str>>, CqlIdentifier<&'de str>>,
        ParsedCqlUserDefinedType<&'de str, CqlIdentifier<&'de str>>,
    >
{
    fn parse(input: &'de str) -> IResult<&'de str, Self, E> {
        alt((
            map(ParsedCqlUserDefinedType::parse, |user_defined_type| {
                CqlStatement::CreateUserDefinedType(user_defined_type)
            }),
            map(CqlTable::parse, |table| CqlStatement::CreateTable(table)),
        ))(input)
    }
}

#[cfg(test)]
mod test {
    use crate::model::cql_type::CqlType;
    use crate::model::order::CqlOrder;
    use crate::model::qualified_identifier::CqlQualifiedIdentifier;
    use crate::model::table::column::CqlColumn;
    use crate::model::table::options::CqlTableOptions;
    use crate::model::table::primary_key::CqlPrimaryKey;
    use crate::model::table::CqlTable;

    use super::*;

    #[test]
    fn test_parse_table() {
        let input = r#"CREATE TABLE IF NOT EXISTS my_keyspace.my_table (
            my_field1 int,
            my_field2 text,
            PRIMARY KEY (my_field1)
        ) WITH CLUSTERING ORDER BY (my_field2 DESC)"#;
        assert_eq!(
            CqlStatement::parse(input),
            Ok::<_, nom::Err<nom::error::Error<_>>>((
                "",
                CqlStatement::CreateTable(CqlTable::new(
                    true,
                    CqlQualifiedIdentifier::new(
                        Some(CqlIdentifier::Unquoted("my_keyspace")),
                        CqlIdentifier::Unquoted("my_table"),
                    ),
                    vec![
                        CqlColumn::new(
                            CqlIdentifier::Unquoted("my_field1"),
                            CqlType::INT,
                            false,
                            false,
                        ),
                        CqlColumn::new(
                            CqlIdentifier::Unquoted("my_field2"),
                            CqlType::TEXT,
                            false,
                            false,
                        ),
                    ],
                    Some(CqlPrimaryKey::new(
                        vec![CqlIdentifier::Unquoted("my_field1")],
                        vec![]
                    )),
                    Some(CqlTableOptions::new(
                        false,
                        vec![(CqlIdentifier::Unquoted("my_field2"), CqlOrder::Desc,)],
                        vec![],
                    )),
                ))
            ))
        )
    }

    #[test]
    fn test_parse_udt() {
        let input = r#"CREATE TYPE IF NOT EXISTS "my_keyspace".my_type (
            my_field1 int,
            my_field2 text,
            my_field3 frozen<list<text>>,
            my_field4 frozen<map<text, text>>,
            my_field5 some_udt
        )"#;
        assert_eq!(
            CqlStatement::parse(input),
            Ok::<_, nom::Err<nom::error::Error<_>>>((
                "",
                CqlStatement::CreateUserDefinedType(ParsedCqlUserDefinedType::new(
                    true,
                    CqlQualifiedIdentifier::new(
                        Some(CqlIdentifier::Quoted("my_keyspace".to_string())),
                        CqlIdentifier::Unquoted("my_type"),
                    ),
                    vec![
                        (CqlIdentifier::Unquoted("my_field1"), CqlType::INT),
                        (CqlIdentifier::Unquoted("my_field2"), CqlType::TEXT),
                        (
                            CqlIdentifier::Unquoted("my_field3"),
                            CqlType::FROZEN(Box::new(CqlType::LIST(Box::new(CqlType::TEXT)))),
                        ),
                        (
                            CqlIdentifier::Unquoted("my_field4"),
                            CqlType::FROZEN(Box::new(CqlType::MAP(Box::new((
                                CqlType::TEXT,
                                CqlType::TEXT
                            ))))),
                        ),
                        (
                            CqlIdentifier::Unquoted("my_field5"),
                            CqlType::UserDefined(CqlIdentifier::Unquoted("some_udt")),
                        ),
                    ]
                ))
            ))
        );
    }
}
