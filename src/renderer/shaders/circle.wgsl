struct FragmentOutput {
    @location(0) outColor: vec4<f32>,
}

var<private> outColor: vec4<f32>;
var<private> gl_FragCoord: vec4<f32>;

struct CameraUniform {
    view_proj: mat4x4<f32>,
};

@group(0) @binding(0) // 1.
var<uniform> camera: CameraUniform;


fn circle(_st: vec2<f32>, _radius: f32) -> f32 {
    var _st_1: vec2<f32>;
    var _radius_1: f32;
    var dist: vec2<f32>;

    _st_1 = _st;
    _radius_1 = _radius;
    let _e4 = _st_1;
    dist = (_e4 - vec2<f32>(0.5));
    let _e10 = _radius_1;
    let _e11 = _radius_1;
    _ = (_e10 - (_e11 * 0.009999999776482582));
    let _e15 = _radius_1;
    let _e16 = _radius_1;
    _ = (_e15 + (_e16 * 0.009999999776482582));
    _ = dist;
    _ = dist;
    let _e22 = dist;
    let _e23 = dist;
    _ = (dot(_e22, _e23) * 4.0);
    let _e27 = _radius_1;
    let _e28 = _radius_1;
    let _e32 = _radius_1;
    let _e33 = _radius_1;
    _ = dist;
    _ = dist;
    let _e39 = dist;
    let _e40 = dist;
    return (1.0 - smoothstep((_e27 - (_e28 * 0.009999999776482582)), (_e32 + (_e33 * 0.009999999776482582)), (dot(_e39, _e40) * 4.0)));
}

fn main_1() {
    var st: vec2<f32>;
    var color: vec3<f32>;

    let _e2 = gl_FragCoord;
    st = (_e2.xy / vec2<f32>(500.0));
    _ = st;
    let _e10 = st;
    let _e12 = circle(_e10, 0.8999999761581421);
    color = vec3<f32>(_e12);
    let _e15 = color;
    outColor = vec4<f32>(_e15.x, _e15.y, _e15.z, 1.0);
    return;
}

@fragment 
fn fs_main(@builtin(position) param: vec4<f32>) -> FragmentOutput {
//    gl_FragCoord = param;
//    main_1();
//    let _e5 = outColor;
//    return FragmentOutput(_e5);
    return FragmentOutput(vec4<f32>(f32(1.0)));
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
};

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec3<f32>,
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
//    out.color = vec3<f32>(1.0,1.0,1.0);
    out.position = camera.view_proj * vec4<f32>(model.position.x, model.position.y,f32(0), f32(1)); // 2.
    return out;
}
