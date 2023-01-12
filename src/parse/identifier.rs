use crate::model::identifier::CqlIdentifier;
use crate::parse::Parse;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until, take_while1};
use nom::character::complete::alpha1;
use nom::error::ParseError;
use nom::{AsChar, IResult, InputTake};

impl<'de, E: ParseError<&'de str>> Parse<&'de str, E> for CqlIdentifier<&'de str> {
    fn parse(input: &'de str) -> IResult<&'de str, Self, E> {
        fn parse_quoted<'de, E: ParseError<&'de str>>(
            input: &'de str,
        ) -> IResult<&str, CqlIdentifier<&'de str>, E> {
            let (input, _) = tag("\"")(input)?;
            let mut acc = String::new();
            let mut input = input;
            loop {
                let (i, s) = take_until("\"")(input)?;
                acc.push_str(s);
                let (i, _) = tag("\"")(i)?;
                input = i;
                if !i.starts_with("\"") {
                    break;
                }
                acc.push('"');
            }

            Ok((input, CqlIdentifier::Quoted(acc)))
        }

        fn parse_unquoted<'de, E: ParseError<&'de str>>(
            input: &'de str,
        ) -> IResult<&str, CqlIdentifier<&'de str>, E> {
            let (i, first) = alpha1(input)?;
            let (i, rest) = take_while1(|c: char| c.is_alpha() || c.is_dec_digit() || c == '_')(i)?;
            Ok((
                i,
                CqlIdentifier::Unquoted(input.take(first.len() + rest.len())),
            ))
        }

        alt((parse_quoted, parse_unquoted))(input)
    }
}
