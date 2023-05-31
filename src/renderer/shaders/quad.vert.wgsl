struct CameraUniform {
    view_proj: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexOutput {
    @builtin(position) member: vec4<f32>,
}

var<private> aPosition_1: vec2<f32>;
var<private> gl_Position: vec4<f32>;

fn main_1() {
    let _e2: vec2<f32> = aPosition_1;
    gl_Position = vec4<f32>(_e2.x, _e2.y, f32(0), f32(1));
    return;
}

@vertex
fn main(@location(0) aPosition: vec2<f32>) -> VertexOutput {
    aPosition_1 = aPosition;
    main_1();
    let _e5: vec4<f32> = gl_Position;
    let out = camera.view_proj * _e5; // 2.
    return VertexOutput(out);
}