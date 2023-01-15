use crate::model::identifier::CqlIdentifier;
use crate::model::table::primary_key::CqlPrimaryKey;
use crate::parse::Parse;
use crate::utils::{space0_around, space0_between};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::error::ParseError;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::IResult;

impl<'de, E: ParseError<&'de str>> Parse<&'de str, E> for CqlPrimaryKey<CqlIdentifier<&'de str>> {
    fn parse(input: &'de str) -> IResult<&'de str, Self, E> {
        let (input, (_, partition_key, clustering_columns, _)) = space0_between((
            tag("("),
            alt((
                map(CqlIdentifier::parse, |name| vec![name]),
                delimited(
                    tag("("),
                    separated_list1(tag(","), space0_around(CqlIdentifier::parse)),
                    tag(")"),
                ),
            )),
            opt(space0_between((
                tag(","),
                separated_list1(tag(","), space0_around(CqlIdentifier::parse)),
            ))),
            tag(")"),
        ))(input)?;

        Ok((
            input,
            CqlPrimaryKey::new(
                partition_key,
                clustering_columns
                    .map(|(_, clustering_columns)| clustering_columns)
                    .unwrap_or_default(),
            ),
        ))
    }
}
