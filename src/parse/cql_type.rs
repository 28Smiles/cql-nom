use crate::model::cql_type::CqlType;
use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case};
use nom::combinator::map;
use nom::error::ParseError;
use nom::IResult;
use nom::multi::separated_list1;
use crate::model::identifier::CqlIdentifier;
use crate::parse::Parse;
use crate::utils::{angle_bracket, seperated, space0_around};

impl<'de, E: ParseError<&'de str>> Parse<&'de str, E> for CqlType<CqlIdentifier<&'de str>>
{
    fn parse(input: &'de str) -> IResult<&'de str, Self, E> {
        alt((
            alt((
                map(tag_no_case("ASCII"), |_| Self::ASCII),
                map(tag_no_case("BIGINT"), |_| Self::BIGINT),
                map(tag_no_case("BLOB"), |_| Self::BLOB),
                map(tag_no_case("BOOLEAN"), |_| Self::BOOLEAN),
                map(tag_no_case("COUNTER"), |_| Self::COUNTER),
                map(tag_no_case("DATE"), |_| Self::DATE),
                map(tag_no_case("DECIMAL"), |_| Self::DECIMAL),
                map(tag_no_case("DOUBLE"), |_| Self::DOUBLE),
                map(tag_no_case("DURATION"), |_| Self::DURATION),
                map(tag_no_case("FLOAT"), |_| Self::FLOAT),
                map(tag_no_case("INET"), |_| Self::INET),
                map(tag_no_case("INT"), |_| Self::INT),
                map(tag_no_case("SMALLINT"), |_| Self::SMALLINT),
                map(tag_no_case("TEXT"), |_| Self::TEXT),
                map(tag_no_case("TIMESTAMP"), |_| Self::TIMESTAMP),
                map(tag_no_case("TIMEUUID"), |_| Self::TIMEUUID),
                map(tag_no_case("TIME"), |_| Self::TIME),
                map(tag_no_case("TINYINT"), |_| Self::TINYINT),
                map(tag_no_case("UUID"), |_| Self::UUID),
                map(tag_no_case("VARCHAR"), |_| Self::VARCHAR),
                map(tag_no_case("VARINT"), |_| Self::VARINT),
            )),
            alt((
                map(
                    // FROZEN '<' cql_type '>'
                    angle_bracket(tag_no_case("FROZEN"), Self::parse),
                    |(_, ty)| Self::FROZEN(Box::new(ty)),
                ),
                map(
                    // MAP '<' (cql_type ',' cql_type) '>'
                    angle_bracket(
                        tag_no_case("MAP"),
                        // cql_type ',' cql_type
                        seperated(
                            Self::parse,
                            tag(","),
                            Self::parse,
                        ),
                    ),
                    |(_, (key, _, value))| Self::MAP(Box::new((key, value))),
                ),
                map(
                    // SET '<' cql_type '>'
                    angle_bracket(tag_no_case("SET"), Self::parse),
                    |(_, ty)| Self::SET(Box::new(ty)),
                ),
                map(
                    // LIST '<' cql_type '>'
                    angle_bracket(tag_no_case("LIST"), Self::parse),
                    |(_, ty)| Self::LIST(Box::new(ty)),
                ),
                map(
                    // TUPLE '<' cql_type ( ',' cql_type )* '>'
                    angle_bracket(
                        tag_no_case("TUPLE"),
                        // cql_type ( ',' cql_type )*
                        separated_list1(tag(","), space0_around(Self::parse)),
                    ),
                    |(_, ty)| Self::TUPLE(ty),
                ),
                map(CqlIdentifier::parse, |ident| Self::UserDefined(ident)),
            )),
        ))(input)
    }
}

#[cfg(test)]
mod test {
    use crate::model::identifier::CqlIdentifier;
    use super::*;

    #[test]
    fn test_parse_type_ascii() {
        let input = "ASCII";
        let result: IResult<_, _, nom::error::Error<&str>> = CqlType::parse(input);
        assert_eq!(result, Ok(("", CqlType::ASCII)));
    }

    #[test]
    fn test_parse_type_bigint() {
        let input = "BIGINT";
        let result: IResult<_, _, nom::error::Error<&str>> = CqlType::parse(input);
        assert_eq!(result, Ok(("", CqlType::BIGINT)));
    }

    #[test]
    fn test_parse_type_blob() {
        let input = "BLOB";
        let result: IResult<_, _, nom::error::Error<&str>> = CqlType::parse(input);
        assert_eq!(result, Ok(("", CqlType::BLOB)));
    }

    #[test]
    fn test_parse_type_boolean() {
        let input = "BOOLEAN";
        let result: IResult<_, _, nom::error::Error<&str>> = CqlType::parse(input);
        assert_eq!(result, Ok(("", CqlType::BOOLEAN)));
    }

    #[test]
    fn test_parse_type_counter() {
        let input = "COUNTER";
        let result: IResult<_, _, nom::error::Error<&str>> = CqlType::parse(input);
        assert_eq!(result, Ok(("", CqlType::COUNTER)));
    }

    #[test]
    fn test_parse_type_date() {
        let input = "DATE";
        let result: IResult<_, _, nom::error::Error<&str>> = CqlType::parse(input);
        assert_eq!(result, Ok(("", CqlType::DATE)));
    }

    #[test]
    fn test_parse_type_decimal() {
        let input = "DECIMAL";
        let result: IResult<_, _, nom::error::Error<&str>> = CqlType::parse(input);
        assert_eq!(result, Ok(("", CqlType::DECIMAL)));
    }

    #[test]
    fn test_parse_type_double() {
        let input = "DOUBLE";
        let result: IResult<_, _, nom::error::Error<&str>> = CqlType::parse(input);
        assert_eq!(result, Ok(("", CqlType::DOUBLE)));
    }

    #[test]
    fn test_parse_type_duration() {
        let input = "DURATION";
        let result: IResult<_, _, nom::error::Error<&str>> = CqlType::parse(input);
        assert_eq!(result, Ok(("", CqlType::DURATION)));
    }

    #[test]
    fn test_parse_type_float() {
        let input = "FLOAT";
        let result: IResult<_, _, nom::error::Error<&str>> = CqlType::parse(input);
        assert_eq!(result, Ok(("", CqlType::FLOAT)));
    }

    #[test]
    fn test_parse_type_inet() {
        let input = "INET";
        let result: IResult<_, _, nom::error::Error<&str>> = CqlType::parse(input);
        assert_eq!(result, Ok(("", CqlType::INET)));
    }

    #[test]
    fn test_parse_type_int() {
        let input = "INT";
        let result: IResult<_, _, nom::error::Error<&str>> = CqlType::parse(input);
        assert_eq!(result, Ok(("", CqlType::INT)));
    }

    #[test]
    fn test_parse_type_smallint() {
        let input = "SMALLINT";
        let result: IResult<_, _, nom::error::Error<&str>> = CqlType::parse(input);
        assert_eq!(result, Ok(("", CqlType::SMALLINT)));
    }

    #[test]
    fn test_parse_type_text() {
        let input = "TEXT";
        let result: IResult<_, _, nom::error::Error<&str>> = CqlType::parse(input);
        assert_eq!(result, Ok(("", CqlType::TEXT)));
    }

    #[test]
    fn test_parse_type_time() {
        let input = "TIME";
        let result: IResult<_, _, nom::error::Error<&str>> = CqlType::parse(input);
        assert_eq!(result, Ok(("", CqlType::TIME)));
    }

    #[test]
    fn test_parse_type_timestamp() {
        let input = "TIMESTAMP";
        let result: IResult<_, _, nom::error::Error<&str>> = CqlType::parse(input);
        assert_eq!(result, Ok(("", CqlType::TIMESTAMP)));
    }

    #[test]
    fn test_parse_type_timeuuid() {
        let input = "TIMEUUID";
        let result: IResult<_, _, nom::error::Error<&str>> = CqlType::parse(input);
        assert_eq!(result, Ok(("", CqlType::TIMEUUID)));
    }

    #[test]
    fn test_parse_type_tinyint() {
        let input = "TINYINT";
        let result: IResult<_, _, nom::error::Error<&str>> = CqlType::parse(input);
        assert_eq!(result, Ok(("", CqlType::TINYINT)));
    }

    #[test]
    fn test_parse_type_uuid() {
        let input = "UUID";
        let result: IResult<_, _, nom::error::Error<&str>> = CqlType::parse(input);
        assert_eq!(result, Ok(("", CqlType::UUID)));
    }

    #[test]
    fn test_parse_type_varchar() {
        let input = "VARCHAR";
        let result: IResult<_, _, nom::error::Error<&str>> = CqlType::parse(input);
        assert_eq!(result, Ok(("", CqlType::VARCHAR)));
    }

    #[test]
    fn test_parse_type_varint() {
        let input = "VARINT";
        let result: IResult<_, _, nom::error::Error<&str>> = CqlType::parse(input);
        assert_eq!(result, Ok(("", CqlType::VARINT)));
    }

    #[test]
    fn test_parse_type_frozen() {
        let input = "FROZEN<INT>";
        let result: IResult<_, _, nom::error::Error<&str>> = CqlType::parse(input);
        assert_eq!(result, Ok(("", CqlType::FROZEN(Box::new(CqlType::INT)))));
    }

    #[test]
    fn test_parse_type_map() {
        let input = "MAP<INT, TEXT>";
        let result: IResult<_, _, nom::error::Error<&str>> = CqlType::parse(input);
        assert_eq!(
            result,
            Ok((
                "",
                CqlType::MAP(Box::new((CqlType::INT, CqlType::TEXT)))
            ))
        );
    }

    #[test]
    fn test_parse_type_set() {
        let input = "SET<INT>";
        let result: IResult<_, _, nom::error::Error<&str>> = CqlType::parse(input);
        assert_eq!(result, Ok(("", CqlType::SET(Box::new(CqlType::INT)))));
    }

    #[test]
    fn test_parse_type_list() {
        let input = "LIST<INT>";
        let result: IResult<_, _, nom::error::Error<&str>> = CqlType::parse(input);
        assert_eq!(result, Ok(("", CqlType::LIST(Box::new(CqlType::INT)))));
    }

    #[test]
    fn test_parse_type_tuple() {
        let input = "TUPLE<INT, TEXT>";
        let result: IResult<_, _, nom::error::Error<&str>> = CqlType::parse(input);
        assert_eq!(
            result,
            Ok((
                "",
                CqlType::TUPLE(vec![CqlType::INT, CqlType::TEXT])
            ))
        );
    }

    #[test]
    fn test_parse_type_udt() {
        let input = "user_defined_type";
        let result: IResult<_, _, nom::error::Error<&str>> = CqlType::parse(input);
        assert_eq!(
            result,
            Ok((
                "",
                CqlType::UserDefined(CqlIdentifier::Unquoted("user_defined_type"))
            ))
        );
    }
}