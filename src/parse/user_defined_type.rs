use crate::model::cql_type::CqlType;
use crate::model::identifier::CqlIdentifier;
use crate::model::qualified_identifier::CqlQualifiedIdentifier;
use crate::model::user_defined_type::CqlUserDefinedType;
use crate::parse::Parse;
use crate::utils::{space1_before, space1_tags_no_case};
use nom::bytes::complete::tag;
use nom::character::complete::{multispace0, multispace1};
use nom::combinator::opt;
use nom::error::ParseError;
use nom::multi::separated_list0;
use nom::sequence::delimited;
use nom::IResult;

impl<'de, E: ParseError<&'de str>> Parse<&'de str, E>
    for CqlUserDefinedType<&'de str, CqlIdentifier<&'de str>>
{
    fn parse(input: &'de str) -> IResult<&'de str, Self, E> {
        let (input, _) = space1_tags_no_case(["CREATE", "TYPE"])(input)?;

        let (input, if_not_exists) =
            opt(space1_before(space1_tags_no_case(["IF", "NOT", "EXISTS"])))(input)?;
        let if_not_exists = if_not_exists.is_some();

        let (input, _) = multispace1(input)?;
        let (input, name) = CqlQualifiedIdentifier::parse(input)?;

        let (input, _) = multispace0(input)?;

        fn parse_field<'de, E: ParseError<&'de str>>(
            input: &'de str,
        ) -> IResult<&'de str, (CqlIdentifier<&'de str>, CqlType<CqlIdentifier<&'de str>>), E>
        {
            let (input, _) = multispace0(input)?;
            let (input, name) = CqlIdentifier::parse(input)?;
            let (input, _) = multispace1(input)?;
            let (input, ty) = CqlType::parse(input)?;
            let (input, _) = multispace0(input)?;

            Ok((input, (name, ty)))
        }

        let (input, fields) =
            delimited(tag("("), separated_list0(tag(","), parse_field), tag(")"))(input)?;

        Ok((input, CqlUserDefinedType::new(if_not_exists, name, fields)))
    }
}
