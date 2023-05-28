use nom::bytes::complete::is_not;
use nom::character::complete::char;
use nom::sequence::delimited;
use nom::IResult;

pub(crate) mod colors;
pub(crate) mod content;
pub(crate) mod params;
pub(crate) mod spacing;

pub fn parse_element_type(input: &str) -> IResult<&str, &str> {
    delimited(char('<'), is_not(" "), char(' '))(input)
}
