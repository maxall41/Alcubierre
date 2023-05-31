struct FragmentOutput {
    @location(0) fColor: vec4<f32>,
}


@fragment
fn main() -> FragmentOutput {
    let color = vec4<f32>(f32(1));
    return FragmentOutput(color);
}