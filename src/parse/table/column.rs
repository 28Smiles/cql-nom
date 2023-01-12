use crate::model::cql_type::CqlType;
use crate::model::identifier::CqlIdentifier;
use crate::model::table::column::CqlColumn;
use crate::parse::Parse;
use crate::utils::{space0_between, space1_before, space1_tags_no_case};
use nom::bytes::complete::tag_no_case;
use nom::combinator::opt;
use nom::error::ParseError;
use nom::IResult;

impl<'de, E: ParseError<&'de str>> Parse<&'de str, E>
    for CqlColumn<&'de str, CqlIdentifier<&'de str>>
{
    fn parse(input: &'de str) -> IResult<&'de str, Self, E> {
        let (input, (name, cql_type)) =
            space0_between((CqlIdentifier::parse, CqlType::parse))(input)?;
        let (input, is_static) = opt(space1_before(tag_no_case("STATIC")))(input)?;
        let (input, is_primary_key) =
            opt(space1_before(space1_tags_no_case(["PRIMARY", "KEY"])))(input)?;

        Ok((
            input,
            CqlColumn::new(
                name,
                cql_type,
                is_static.is_some(),
                is_primary_key.is_some(),
            ),
        ))
    }
}
