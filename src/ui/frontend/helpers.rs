#[macro_export]
macro_rules! show_compiler_error {
    // `()` indicates that the macro takes no argument.
    ($value: expr,$line: expr,$msg: ident,$help: ident,$pos: expr) => {{
        use miette::LabeledSpan;
        match $value {
            Ok(v) => v,
            Err(_e) => {
                let source = $line.to_string();
                let report = miette::Report::from({
                    let mut diag = ::miette::MietteDiagnostic::new({
                        let res = $msg;
                        res
                    });
                    diag.labels = Some(vec![LabeledSpan::at($pos..$pos, $help)]);
                    diag.help = Some($help.into());
                    diag
                })
                .with_source_code(source);
                panic!("{:?}", report)
            }
        }
    }};
}
