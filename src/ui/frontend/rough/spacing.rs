use crate::ui::frontend::SpacingUnit;
use nom::branch::alt;
use nom::bytes::complete::take_until;
use nom::combinator::{map, recognize};
use nom::IResult;

fn take_until_pixels(input: &str) -> IResult<&str, SpacingUnit> {
    map(recognize(take_until("px")), |s: &str| {
        SpacingUnit::Pixels(s.parse().unwrap())
    })(input)
}

fn take_until_percent_height(input: &str) -> IResult<&str, SpacingUnit> {
    map(recognize(take_until("%h")), |s: &str| {
        SpacingUnit::PercentHeight(s.parse().unwrap())
    })(input)
}

fn take_until_percent_width(input: &str) -> IResult<&str, SpacingUnit> {
    map(recognize(take_until("%w")), |s: &str| {
        SpacingUnit::PercentWidth(s.parse().unwrap())
    })(input)
}

pub fn parse_spacing_units(input: &str) -> IResult<&str, SpacingUnit> {
    alt((
        take_until_pixels,
        take_until_percent_width,
        take_until_percent_height,
    ))(input)
}
