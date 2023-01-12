mod column;
mod options;
mod primary_key;

use crate::model::identifier::CqlIdentifier;
use crate::model::qualified_identifier::CqlQualifiedIdentifier;
use crate::model::table::column::CqlColumn;
use crate::model::table::options::CqlTableOptions;
use crate::model::table::primary_key::CqlPrimaryKey;
use crate::model::table::CqlTable;
use crate::parse::Parse;
use crate::utils::{
    space0_around, space0_between, space1_before, space1_between, space1_tags_no_case,
};
use nom::bytes::complete::{tag, tag_no_case};
use nom::character::complete::multispace0;
use nom::combinator::opt;
use nom::error::ParseError;
use nom::multi::separated_list0;
use nom::IResult;

impl<'de, E: ParseError<&'de str>> Parse<&'de str, E>
    for CqlTable<&'de str, CqlColumn<&'de str, CqlIdentifier<&'de str>>, CqlIdentifier<&'de str>>
{
    fn parse(input: &'de str) -> IResult<&'de str, Self, E> {
        let (input, _) = space1_tags_no_case(["CREATE", "TABLE"])(input)?;
        let (input, if_not_exists) =
            opt(space1_before(space1_tags_no_case(["IF", "NOT", "EXISTS"])))(input)?;
        let (input, name) = space1_before(CqlQualifiedIdentifier::parse)(input)?;
        let (input, _) = multispace0(input)?;
        let (input, _) = tag("(")(input)?;
        let (input, columns) = separated_list0(tag(","), space0_around(CqlColumn::parse))(input)?;
        let (input, primary_key) = opt(space0_between((
            tag(","),
            space1_tags_no_case(["PRIMARY", "KEY"]),
            CqlPrimaryKey::parse,
        )))(input)?;
        let (input, _) = multispace0(input)?;
        let (input, _) = tag(")")(input)?;
        let (input, _) = multispace0(input)?;
        let (input, options) = opt(space1_between((
            tag_no_case("WITH"),
            CqlTableOptions::parse,
        )))(input)?;

        Ok((
            input,
            CqlTable::new(
                if_not_exists.is_some(),
                name,
                columns,
                primary_key.map(|(_, _, pk)| pk),
                options.map(|(_, options)| options),
            ),
        ))
    }
}
