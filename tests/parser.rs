use std::fs;
use flame::ui::frontend::Element::{Button, Text};
use flame::ui::frontend::ElementAlignment::TopLeft;
use flame::ui::frontend::SpacingUnit::{PercentWidth, Pixels, PercentHeight};
use flame::ui::frontend::ValueOrVar::{Value, Variable};
use flame::ui::frontend::{
    ButtonElement, ButtonStyleData, HyperFoilAST, RGBColor, TextElement, TextStyleData,
};
use flame::ui::parse_file;

#[test]
fn basic_rough_parse() {
    let basic_res = HyperFoilAST {
        elements: vec![
            Text(TextElement {
                content: Value("Im on the left!".to_string()),
                styles: TextStyleData {
                    font: None,
                    font_size: None,
                    alignment: Some(TopLeft),
                    color: Some(RGBColor {
                        red: 250,
                        green: 17,
                        blue: 234,
                    }),
                    margin_top: Some(Pixels(2)),
                    margin_bottom: None,
                    margin_left: Some(PercentWidth(2)),
                    margin_right: None,
                },
                classes: vec!["sel2".to_string(), "xyz".to_string()],
            }),
            Button(ButtonElement {
                content: Value("Decrement Health".to_string()),
                styles: ButtonStyleData {
                    font: None,
                    font_size_px: None,
                    alignment: Some(TopLeft),
                    color: None,
                    background_color: None,
                    margin_top: None,
                    margin_bottom: None,
                    margin_left: None,
                    margin_right: None,
                    width: Some(Pixels(40)),
                    height: None,
                },
                classes: vec!["b1".to_string(), "cubes".to_string()],
                binding: "dec_health".to_string(),
            }),
            Text(TextElement {
                content: Variable("Health".to_string()),
                styles: TextStyleData {
                    font: None,
                    font_size: None,
                    alignment: None,
                    color: None,
                    margin_top: None,
                    margin_bottom: None,
                    margin_left: None,
                    margin_right: None,
                },
                classes: vec!["sel1".to_string()],
            }),
        ],
    };

    assert_eq!(parse_file("tests/basic.hfm"), basic_res);
}

#[test]
fn somewhat_complex_rough_parse() {
    let c1 = fs::read_to_string("tests/c1.json").expect("Should have been able to read the file");

    let c1_ast: HyperFoilAST = serde_json::from_str(&c1).unwrap();

    assert_eq!(parse_file("tests/c1.hfm"), c1_ast);
}

#[test]
fn more_complex_rough_parse() {
    let c2 = fs::read_to_string("tests/c2.json").expect("Should have been able to read the file");

    let c2_ast: HyperFoilAST = serde_json::from_str(&c2).unwrap();

    assert_eq!(parse_file("tests/c2.hfm"), c2_ast);
}
