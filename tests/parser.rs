use alcubierre::ui::frontend::Element::{Button, Text};
use alcubierre::ui::frontend::ElementAlignment::TopLeft;
use alcubierre::ui::frontend::SpacingUnit::{PercentHeight, PercentWidth, Pixels};
use alcubierre::ui::frontend::ValueOrVar::{Value, Variable};
use alcubierre::ui::frontend::{
    ButtonElement, ButtonStyleData, ElementAlignment, HyperFoilAST, RGBColor, TextElement,
    TextStyleData,
};
use alcubierre::ui::parse_ui_blob;
use pretty_assertions::assert_eq;
use std::fs;
use std::fs::File;

#[test]
fn basic_rough_parse() {
    let basic_res = HyperFoilAST {
        elements: vec![
            Text(TextElement {
                content: Value("Im on the left!".to_string()),
                styles: TextStyleData {
                    font: "".to_string(),
                    font_size: Pixels(12),
                    alignment: TopLeft,
                    color: RGBColor {
                        red: 250,
                        green: 17,
                        blue: 234,
                    },
                    margin_top: Pixels(2),
                    margin_bottom: Pixels(0),
                    margin_left: PercentWidth(2),
                    margin_right: Pixels(0),
                },
                classes: vec!["sel2".to_string(), "xyz".to_string()],
            }),
            Button(ButtonElement {
                content: Value("Decrement Health".to_string()),
                styles: ButtonStyleData {
                    font: "".to_string(),
                    font_size_px: Pixels(12),
                    alignment: TopLeft,
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
                    margin_top: Pixels(0),
                    margin_bottom: Pixels(0),
                    margin_left: Pixels(0),
                    margin_right: Pixels(0),
                    width: Pixels(40),
                    height: Pixels(50),
                },
                classes: vec!["b1".to_string(), "cubes".to_string()],
                binding: "dec_health".to_string(),
            }),
            Text(TextElement {
                content: Variable(("Health".to_string(), "Health: ".to_string())),
                styles: TextStyleData {
                    font: "".to_string(),
                    font_size: Pixels(12),
                    alignment: ElementAlignment::TopLeft,
                    color: RGBColor {
                        red: 255,
                        green: 255,
                        blue: 255,
                    },
                    margin_top: Pixels(0),
                    margin_bottom: Pixels(0),
                    margin_left: Pixels(0),
                    margin_right: Pixels(0),
                },
                classes: vec!["sel1".to_string()],
            }),
        ],
    };

    assert_eq!(parse_ui_blob(include_str!("./basic.hfm")), basic_res);
}

#[test]
fn somewhat_complex_rough_parse() {
    let c1 = fs::read_to_string("./c1.json").expect("Should have been able to read the file");

    let c1_ast: HyperFoilAST = serde_json::from_str(&c1).unwrap();

    assert_eq!(parse_ui_blob(include_str!("./c1.hfm")), c1_ast);

    // let ast = parse_ui_blob(include_str!("./c1.hfm"));

    // let st = serde_json::to_string(&ast).unwrap();

    // fs::write("c1.json", st).expect("Unable to write file");
}

#[test]
fn more_complex_rough_parse() {
    let c2 = fs::read_to_string("./c2.json").expect("Should have been able to read the file");

    let c2_ast: HyperFoilAST = serde_json::from_str(&c2).unwrap();

    assert_eq!(parse_ui_blob(include_str!("./c2.hfm")), c2_ast);

    // let ast = parse_ui_blob(include_str!("./c2.hfm"));

    // let st = serde_json::to_string(&ast).unwrap();

    // fs::write("c2.json", st).expect("Unable to write file");
}
