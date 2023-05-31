struct FragmentOutput {
    @location(0) fColor: vec4<f32>,
}

var<private> fColor: vec4<f32>;

fn main_1() {
    fColor = vec4<f32>(f32(1));
    return;
}

@fragment
fn main() -> FragmentOutput {
    main_1();
    let _e3: vec4<f32> = fColor;
    return FragmentOutput(_e3);
}