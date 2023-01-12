use nom::IResult;

mod cql_type;
mod identifier;
mod qualified_identifier;
mod user_defined_type;
mod table;
mod statement;

pub trait Parse<I, E> {
    /// A parser takes in input type, and returns a `Result` containing
    /// either the remaining input and the output value, or an error
    fn parse(input: I) -> IResult<I, Self, E>
    where
        Self: Sized;
}
