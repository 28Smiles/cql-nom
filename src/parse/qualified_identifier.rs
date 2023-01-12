use crate::model::identifier::CqlIdentifier;
use crate::model::qualified_identifier::CqlQualifiedIdentifier;
use crate::parse::Parse;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::combinator::opt;
use nom::error::ParseError;
use nom::IResult;

impl<'de, E: ParseError<&'de str>> Parse<&'de str, E> for CqlQualifiedIdentifier<&'de str> {
    fn parse(input: &'de str) -> IResult<&'de str, CqlQualifiedIdentifier<&'de str>, E> {
        let (input, name_or_keyspace) = CqlIdentifier::parse(input)?;
        let (input, _) = multispace0(input)?;
        let (input, dot) = opt(tag("."))(input)?;

        if dot.is_some() {
            let (input, _) = multispace0(input)?;
            let (input, name) = CqlIdentifier::parse(input)?;
            Ok((
                input,
                CqlQualifiedIdentifier::new(Some(name_or_keyspace), name),
            ))
        } else {
            Ok((input, CqlQualifiedIdentifier::new(None, name_or_keyspace)))
        }
    }
}
