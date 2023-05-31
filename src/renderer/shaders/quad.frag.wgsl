struct FragmentOutput {
    @location(0) fColor: vec4<f32>,
}

//var<private> fColor: vec4<f32>;
//
//fn main_1() {
//    fColor = vec4<f32>(f32(1));
//    return;
//}

@group(0) @binding(1) // 1.
var<uniform> color: vec4<f32>;


@fragment
fn main() -> FragmentOutput {
//    main_1();
//    let _e3: vec4<f32> = color;
    return FragmentOutput(color);
}