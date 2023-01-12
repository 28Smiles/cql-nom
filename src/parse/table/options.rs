use crate::model::identifier::CqlIdentifier;
use crate::model::order::CqlOrder;
use crate::model::table::options::CqlTableOptions;
use crate::parse::Parse;
use crate::utils::{space0_around, space0_between, space1_before, space1_between, space1_tags};
use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case};
use nom::character::complete::multispace0;
use nom::combinator::{map, opt};
use nom::error::ParseError;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::IResult;

impl<'de, E: ParseError<&'de str>> Parse<&'de str, E>
    for CqlTableOptions<&'de str, CqlIdentifier<&'de str>>
{
    fn parse(input: &'de str) -> IResult<&'de str, Self, E> {
        let mut input = input;
        let mut compact_storage = false;
        let mut clustering_order = None;
        let mut options = Vec::new();

        loop {
            let (i, _) = multispace0(input)?;
            let (i, option) = opt(|input| {
                alt((
                    map(space1_tags(["COMPACT", "STORAGE"]), |_| {
                        compact_storage = true;
                    }),
                    map(
                        space0_between((
                            space1_tags(["CLUSTERING", "ORDER", "BY"]),
                            delimited(
                                tag("("),
                                separated_list1(
                                    tag(","),
                                    space0_around(space1_between((
                                        CqlIdentifier::parse,
                                        alt((
                                            map(tag_no_case("ASC"), |_| CqlOrder::Asc),
                                            map(tag_no_case("DESC"), |_| CqlOrder::Desc),
                                        )),
                                    ))),
                                ),
                                tag(")"),
                            ),
                        )),
                        |order| {
                            clustering_order = Some(order);
                        },
                    ),
                    // TODO: parse options.
                ))(input)
            })(i)?;

            if option.is_none() {
                input = i;
                break;
            }

            let (i, option) = opt(space1_before(tag_no_case("AND")))(i)?;

            if option.is_none() {
                input = i;
                break;
            }

            input = i;
        }

        Ok((
            input,
            CqlTableOptions::new(
                compact_storage,
                clustering_order
                    .map(|(_, clustering_order)| clustering_order)
                    .unwrap_or_default(),
                options,
            ),
        ))
    }
}
