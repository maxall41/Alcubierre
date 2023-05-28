use crate::ui::frontend::ValueOrVar;
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, take_until};
use nom::character::complete::char;
use nom::combinator::recognize;
use nom::sequence::delimited;
use nom::Err::Error;
use nom::IResult;

fn strip_pre_content(input: &str) -> IResult<&str, &str> {
    recognize(take_until(">"))(input)
}

fn strip_pre_insert_tag(input: &str) -> IResult<&str, &str> {
    recognize(take_until("?"))(input)
}

fn no_pre_insert_tag(input: &str) -> IResult<&str, &str> {
    Ok((input, ""))
}

fn parse_insert_tag(input: &str) -> IResult<&str, &str> {
    alt((strip_pre_insert_tag, no_pre_insert_tag))(input)
}

fn extract_content_from_stripped(input: &str) -> IResult<&str, &str> {
    delimited(char('>'), is_not("<"), tag("<"))(input)
}

pub fn get_content(input: &str) -> ValueOrVar {
    let stripped = strip_pre_content(input).unwrap().0;
    let content = extract_content_from_stripped(stripped).unwrap().1;
    let content_tag_parsed = parse_insert_tag(content).unwrap().0;

    return if content_tag_parsed.starts_with("?") {
        ValueOrVar::Variable(content_tag_parsed.to_string().replace("?", ""))
    } else {
        ValueOrVar::Value(content_tag_parsed.to_string())
    };
}
