use crate::show_compiler_error;
use crate::ui::frontend::errors::*;
use crate::ui::frontend::rough::colors::hex_color;
use crate::ui::frontend::rough::spacing::parse_spacing_units;
use crate::ui::frontend::{
    ButtonStyleData, ElementAlignment, RGBColor, SpacingUnit, TextStyleData,
};
use miette::miette;
use miette::LabeledSpan;
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{char, multispace0};
use nom::combinator::{map, value};
use nom::multi::many1;
use nom::sequence::{delimited, terminated};
use nom::IResult;

pub struct RoughParamsText {
    pub classes: Vec<String>,
    pub binding: Option<String>,
    pub styles: TextStyleData,
}

pub struct RoughParamsButton {
    pub classes: Vec<String>,
    pub binding: Option<String>,
    pub styles: ButtonStyleData,
}

#[derive(Debug)]
enum Parameter<'a> {
    Classes(&'a str),
    Binding(&'a str),
    BackgroundColor(&'a str),
    Font(&'a str),
    FontSize(&'a str),
    Color(&'a str),
    MarginTop(&'a str),
    MarginBottom(&'a str),
    MarginLeft(&'a str),
    MarginRight(&'a str),
    Alignment(&'a str),
    Width(&'a str),
    Height(&'a str),
}

pub fn starts_with_open_angle_bracket(input: &str) -> IResult<&str, &str> {
    value(input, char('<'))(input)
}

fn get_inner_params(input: &str) -> IResult<&str, &str> {
    delimited(
        delimited(char('<'), is_not(" "), char(' ')),
        is_not(">"),
        char('>'),
    )(input)
}

fn get_classes_from_params(input: &str) -> IResult<&str, Parameter> {
    map(
        delimited(tag(r#"class=""#), is_not(r#"""#), char('"')),
        |s: &str| Parameter::Classes(s),
    )(input)
}

fn get_binding_from_params(input: &str) -> IResult<&str, Parameter> {
    map(
        delimited(tag(r#"bind=""#), is_not(r#"""#), char('"')),
        |s: &str| Parameter::Binding(s),
    )(input)
}

fn get_align_from_params(input: &str) -> IResult<&str, Parameter> {
    map(
        delimited(tag(r#"align=""#), is_not(r#"""#), char('"')),
        |s: &str| Parameter::Alignment(s),
    )(input)
}

fn get_color_from_params(input: &str) -> IResult<&str, Parameter> {
    map(
        delimited(tag(r#"color=""#), is_not(r#"""#), char('"')),
        |s: &str| Parameter::Color(s),
    )(input)
}

fn get_background_color_from_params(input: &str) -> IResult<&str, Parameter> {
    map(
        delimited(tag(r#"bg_color=""#), is_not(r#"""#), char('"')),
        |s: &str| Parameter::BackgroundColor(s),
    )(input)
}

fn get_font_size_from_params(input: &str) -> IResult<&str, Parameter> {
    map(
        delimited(tag(r#"font_size=""#), is_not(r#"""#), char('"')),
        |s: &str| Parameter::FontSize(s),
    )(input)
}

fn get_font_type_from_params(input: &str) -> IResult<&str, Parameter> {
    map(
        delimited(tag(r#"font=""#), is_not(r#"""#), char('"')),
        |s: &str| Parameter::Font(s),
    )(input)
}

// Margins
fn get_margin_bottom_from_params(input: &str) -> IResult<&str, Parameter> {
    map(
        delimited(tag(r#"mb=""#), is_not(r#"""#), char('"')),
        |s: &str| Parameter::MarginBottom(s),
    )(input)
}

fn get_margin_top_from_params(input: &str) -> IResult<&str, Parameter> {
    map(
        delimited(tag(r#"mt=""#), is_not(r#"""#), char('"')),
        |s: &str| Parameter::MarginTop(s),
    )(input)
}

fn get_margin_left_from_params(input: &str) -> IResult<&str, Parameter> {
    map(
        delimited(tag(r#"ml=""#), is_not(r#"""#), char('"')),
        |s: &str| Parameter::MarginLeft(s),
    )(input)
}

fn get_margin_right_from_params(input: &str) -> IResult<&str, Parameter> {
    map(
        delimited(tag(r#"mr=""#), is_not(r#"""#), char('"')),
        |s: &str| Parameter::MarginRight(s),
    )(input)
}

fn get_width_from_params(input: &str) -> IResult<&str, Parameter> {
    map(
        delimited(tag(r#"width=""#), is_not(r#"""#), char('"')),
        |s: &str| Parameter::Width(s),
    )(input)
}

fn get_height_from_params(input: &str) -> IResult<&str, Parameter> {
    map(
        delimited(tag(r#"height=""#), is_not(r#"""#), char('"')),
        |s: &str| Parameter::Height(s),
    )(input)
}

fn get_width_from_params_short(input: &str) -> IResult<&str, Parameter> {
    map(
        delimited(tag(r#"w=""#), is_not(r#"""#), char('"')),
        |s: &str| Parameter::Width(s),
    )(input)
}

fn get_height_from_params_short(input: &str) -> IResult<&str, Parameter> {
    map(
        delimited(tag(r#"h=""#), is_not(r#"""#), char('"')),
        |s: &str| Parameter::Height(s),
    )(input)
}
//

fn get_params_inner_sys(input: &str) -> IResult<&str, Parameter> {
    terminated(
        alt((
            get_binding_from_params,
            get_classes_from_params,
            get_align_from_params,
            get_color_from_params,
            get_font_size_from_params,
            get_margin_left_from_params,
            get_margin_right_from_params,
            get_margin_top_from_params,
            get_margin_bottom_from_params,
            get_background_color_from_params,
            get_font_type_from_params,
            get_width_from_params,
            get_height_from_params,
            get_width_from_params_short,
            get_height_from_params_short,
        )),
        multispace0,
    )(input)
}

fn get_all_params(input: &str) -> IResult<&str, Vec<Parameter>> {
    many1(get_params_inner_sys)(input)
}

fn get_element_alignment(input: &str, line: &str) -> ElementAlignment {
    match input {
        "top-left" => ElementAlignment::TopLeft,
        "top-right" => ElementAlignment::TopRight,
        "bottom-left" => ElementAlignment::BottomLeft,
        "bottom-right" => ElementAlignment::BottomRight,
        "center" => ElementAlignment::CenterVerticalAndHorizontal,
        "center-vertical" => ElementAlignment::CenterVertical,
        "center-horizontal" => ElementAlignment::CenterHorizontal,
        "center-horizontal+center-vertical" => ElementAlignment::CenterVerticalAndHorizontal,
        "center-vertical+center-horizontal" => ElementAlignment::CenterVerticalAndHorizontal,
        _ => {
            let source = line.to_string();
            let apos = source.find(input).unwrap_or(0);
            let report = miette::Report::from({
                let mut diag = miette::MietteDiagnostic::new({
                    let res = UNKNOWN_ALIGNMENT_ERROR;
                    res
                });
                diag.labels = Some(vec![LabeledSpan::at(apos..apos, UNKNOWN_ALIGNMENT_HELP)]);
                diag.help = Some(UNKNOWN_ALIGNMENT_HELP.into());
                diag
            })
            .with_source_code(source);
            panic!("{:?}", report)
        }
    }
}

pub fn extract_params_from_line_for_text(line: &str) -> RoughParamsText {
    let params = show_compiler_error!(
        get_inner_params(line),
        line,
        FAILED_TO_GET_INNER_PARAMS_ERROR,
        FAILED_TO_GET_INNER_PARAMS_HELP,
        0
    )
    .1;

    let bc = show_compiler_error!(
        get_all_params(params),
        line,
        FAILED_TO_GET_ALL_PARAMS_ERROR,
        FAILED_TO_GET_ALL_PARAMS_HELP,
        0
    )
    .1;

    let mut final_params = RoughParamsText {
        binding: None,
        classes: vec![],
        styles: TextStyleData {
            font: "".to_string(),
            font_size: SpacingUnit::Pixels(12),
            alignment: ElementAlignment::TopLeft,
            color: RGBColor {
                red: 255,
                green: 255,
                blue: 255,
            },
            margin_top: SpacingUnit::Pixels(0),
            margin_bottom: SpacingUnit::Pixels(0),
            margin_left: SpacingUnit::Pixels(0),
            margin_right: SpacingUnit::Pixels(0),
        },
    };

    for param in bc {
        match param {
            Parameter::Font(d) => final_params.styles.font = d.to_string(),
            Parameter::FontSize(d) => {
                final_params.styles.font_size = show_compiler_error!(
                    parse_spacing_units(d),
                    line,
                    FAILED_TO_PARSE_UNIT_ERROR,
                    FAILED_TO_PARSE_UNIT_HELP,
                    line.find(d).unwrap_or(0)
                )
                .1;
            }
            Parameter::MarginTop(d) => {
                final_params.styles.margin_top = show_compiler_error!(
                    parse_spacing_units(d),
                    line,
                    FAILED_TO_PARSE_UNIT_ERROR,
                    FAILED_TO_PARSE_UNIT_HELP,
                    line.find(d).unwrap_or(0)
                )
                .1;
            }
            Parameter::MarginBottom(d) => {
                final_params.styles.margin_bottom = show_compiler_error!(
                    parse_spacing_units(d),
                    line,
                    FAILED_TO_PARSE_UNIT_ERROR,
                    FAILED_TO_PARSE_UNIT_HELP,
                    line.find(d).unwrap_or(0)
                )
                .1;
            }
            Parameter::MarginLeft(d) => {
                final_params.styles.margin_left = show_compiler_error!(
                    parse_spacing_units(d),
                    line,
                    FAILED_TO_PARSE_UNIT_ERROR,
                    FAILED_TO_PARSE_UNIT_HELP,
                    line.find(d).unwrap_or(0)
                )
                .1;
            }
            Parameter::MarginRight(d) => {
                final_params.styles.margin_right = show_compiler_error!(
                    parse_spacing_units(d),
                    line,
                    FAILED_TO_PARSE_UNIT_ERROR,
                    FAILED_TO_PARSE_UNIT_HELP,
                    line.find(d).unwrap_or(0)
                )
                .1;
            }
            Parameter::Color(d) => {
                let color = show_compiler_error!(
                    hex_color(d),
                    line,
                    HEX_COLOR_NOT_VALID_ERROR,
                    HEX_COLOR_NOT_VALID_HELP,
                    line.find(d).unwrap_or(0)
                )
                .1;
                final_params.styles.color = color
            }
            Parameter::Alignment(d) => {
                let element_alignment = get_element_alignment(d, line);
                final_params.styles.alignment = element_alignment
            }
            Parameter::Classes(d) => {
                let split_classes = d.split(" ").collect::<Vec<&str>>();
                for class in split_classes {
                    final_params.classes.push(class.to_string());
                }
            }
            _ => {}
        }
    }

    return final_params;
}

pub fn extract_params_from_line_for_button(line: &str) -> RoughParamsButton {
    let params = show_compiler_error!(
        get_inner_params(line),
        line,
        FAILED_TO_GET_INNER_PARAMS_ERROR,
        FAILED_TO_GET_INNER_PARAMS_HELP,
        0
    )
    .1;

    let bc = show_compiler_error!(
        get_all_params(params),
        line,
        FAILED_TO_GET_ALL_PARAMS_ERROR,
        FAILED_TO_GET_ALL_PARAMS_HELP,
        0
    )
    .1;

    let mut final_params = RoughParamsButton {
        binding: None,
        classes: vec![],
        styles: ButtonStyleData {
            font: "".to_string(),
            font_size_px: SpacingUnit::Pixels(12),
            alignment: ElementAlignment::TopLeft,
            color: RGBColor {
                red: 255,
                green: 255,
                blue: 255,
            },
            background_color: RGBColor {
                red: 0,
                green: 0,
                blue: 0,
            },
            margin_top: SpacingUnit::Pixels(0),
            margin_bottom: SpacingUnit::Pixels(0),
            margin_left: SpacingUnit::Pixels(0),
            margin_right: SpacingUnit::Pixels(0),
            width: SpacingUnit::Pixels(100),
            height: SpacingUnit::Pixels(50),
        },
    };

    for param in bc {
        match param {
            Parameter::Font(d) => final_params.styles.font = d.to_string(),
            Parameter::FontSize(d) => {
                final_params.styles.font_size_px = show_compiler_error!(
                    parse_spacing_units(d),
                    line,
                    FAILED_TO_PARSE_UNIT_ERROR,
                    FAILED_TO_PARSE_UNIT_HELP,
                    line.find(d).unwrap_or(0)
                )
                .1;
            }
            Parameter::MarginTop(d) => {
                final_params.styles.margin_top = show_compiler_error!(
                    parse_spacing_units(d),
                    line,
                    FAILED_TO_PARSE_UNIT_ERROR,
                    FAILED_TO_PARSE_UNIT_HELP,
                    line.find(d).unwrap_or(0)
                )
                .1;
            }
            Parameter::MarginBottom(d) => {
                final_params.styles.margin_bottom = show_compiler_error!(
                    parse_spacing_units(d),
                    line,
                    FAILED_TO_PARSE_UNIT_ERROR,
                    FAILED_TO_PARSE_UNIT_HELP,
                    line.find(d).unwrap_or(0)
                )
                .1;
            }
            Parameter::MarginLeft(d) => {
                final_params.styles.margin_left = show_compiler_error!(
                    parse_spacing_units(d),
                    line,
                    FAILED_TO_PARSE_UNIT_ERROR,
                    FAILED_TO_PARSE_UNIT_HELP,
                    line.find(d).unwrap_or(0)
                )
                .1;
            }
            Parameter::MarginRight(d) => {
                final_params.styles.margin_right = show_compiler_error!(
                    parse_spacing_units(d),
                    line,
                    FAILED_TO_PARSE_UNIT_ERROR,
                    FAILED_TO_PARSE_UNIT_HELP,
                    line.find(d).unwrap_or(0)
                )
                .1;
            }
            Parameter::Color(d) => {
                let color = show_compiler_error!(
                    hex_color(d),
                    line,
                    HEX_COLOR_NOT_VALID_ERROR,
                    HEX_COLOR_NOT_VALID_HELP,
                    line.find(d).unwrap_or(0)
                )
                .1;
                final_params.styles.color = color
            }
            Parameter::BackgroundColor(d) => {
                let color = show_compiler_error!(
                    hex_color(d),
                    line,
                    HEX_COLOR_NOT_VALID_ERROR,
                    HEX_COLOR_NOT_VALID_HELP,
                    line.find(d).unwrap_or(0)
                )
                .1;
                final_params.styles.background_color = color
            }
            Parameter::Alignment(d) => {
                let element_alignment = get_element_alignment(d, line);
                final_params.styles.alignment = element_alignment
            }
            Parameter::Classes(d) => {
                let split_classes = d.split(" ").collect::<Vec<&str>>();
                for class in split_classes {
                    final_params.classes.push(class.to_string());
                }
            }
            Parameter::Binding(d) => final_params.binding = Some(d.to_string()),
            Parameter::Height(d) => {
                final_params.styles.height = show_compiler_error!(
                    parse_spacing_units(d),
                    line,
                    FAILED_TO_PARSE_UNIT_ERROR,
                    FAILED_TO_PARSE_UNIT_HELP,
                    line.find(d).unwrap_or(0)
                )
                .1;
            }
            Parameter::Width(d) => {
                final_params.styles.width = show_compiler_error!(
                    parse_spacing_units(d),
                    line,
                    FAILED_TO_PARSE_UNIT_ERROR,
                    FAILED_TO_PARSE_UNIT_HELP,
                    line.find(d).unwrap_or(0)
                )
                .1;
            }
        }
    }

    return final_params;
}
