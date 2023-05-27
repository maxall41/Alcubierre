use nom::{
    IResult,
    error::ParseError,
    combinator::value,
    sequence::pair,
    bytes::complete::is_not,
    character::complete::char,
};

pub fn comment_stripper<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, (), E>
{
    value(
        (), // Output is thrown away.
        pair(char('#'), is_not("\n\r"))
    )(i)
}