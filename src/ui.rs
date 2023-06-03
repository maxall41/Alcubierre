use crate::ui::frontend::errors::{UNKNOWN_ELEMENT_ERROR, UNKNOWN_ELEMENT_HELP};
use crate::ui::frontend::rough::content::get_content;
use crate::ui::frontend::rough::params::{
    extract_params_from_line_for_button, extract_params_from_line_for_text,
    starts_with_open_angle_bracket,
};
use crate::ui::frontend::rough::parse_element_type;
use crate::ui::frontend::{ButtonElement, Element, HyperFoilAST, TextElement};
use miette::LabeledSpan;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub mod backend;
pub mod frontend;

fn parse_rough_line(line: &str) -> Option<Element> {
    let element_type = parse_element_type(line).unwrap().1;
    let content = get_content(line);
    match element_type {
        "button" => {
            let params = extract_params_from_line_for_button(line);
            Some(Element::Button(ButtonElement {
                content,
                styles: params.styles,
                classes: params.classes,
                binding: params.binding.unwrap(),
            }))
        }
        "text" => {
            let params = extract_params_from_line_for_text(line);
            Some(Element::Text(TextElement {
                content,
                styles: params.styles,
                classes: params.classes,
            }))
        }
        _ => {
            let source = line.to_string();
            let report = miette::Report::from({
                let mut diag = miette::MietteDiagnostic::new({
                    let res = UNKNOWN_ELEMENT_ERROR;
                    res
                });
                diag.labels = Some(vec![LabeledSpan::at(0..0, UNKNOWN_ELEMENT_HELP)]);
                diag.help = Some(UNKNOWN_ELEMENT_HELP.into());
                diag
            })
            .with_source_code(source);
            panic!("{:?}", report)
        }
    }
}

pub fn parse_file(file: &str) -> HyperFoilAST {
    let file = File::open(file).unwrap();
    let reader = BufReader::new(file);

    let mut ast = HyperFoilAST { elements: vec![] };

    for line in reader.lines() {
        let l = line.unwrap();
        if !l.starts_with("#") {
            let res = starts_with_open_angle_bracket(l.as_str());
            match res {
                Ok(r) => {
                    let x = parse_rough_line(r.1);
                    ast.elements.push(x.expect("Parsing failed!"));
                }
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        }
    }

    ast
}
